/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package io.github.zeus;

import com.google.common.collect.ImmutableMap;
import io.github.zeus.client.ZeusClient;
import io.github.zeus.rpc.*;
import io.github.zeus.schema.ZeusDB;
import io.github.zeus.schema.ZeusTable;
import org.apache.drill.common.exceptions.ExecutionSetupException;
import org.apache.drill.common.expression.SchemaPath;
import org.apache.drill.common.types.TypeProtos.MajorType;
import org.apache.drill.common.types.TypeProtos.MinorType;
import org.apache.drill.common.types.Types;
import org.apache.drill.exec.exception.SchemaChangeException;
import org.apache.drill.exec.expr.TypeHelper;
import org.apache.drill.exec.ops.OperatorContext;
import org.apache.drill.exec.physical.impl.OutputMutator;
import org.apache.drill.exec.record.MaterializedField;
import org.apache.drill.exec.store.AbstractRecordReader;
import org.apache.drill.exec.vector.*;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.net.InetAddress;
import java.nio.ByteBuffer;
import java.util.*;
import java.util.stream.Collectors;

import static io.github.zeus.rpc.ColumnType.*;

public class ZeusRecordReader extends AbstractRecordReader {
  private static final Logger logger = LoggerFactory.getLogger(ZeusRecordReader.class);
  private static final int MAX_BATCH_COUNT = 4000;
  private static final String hostname;

  static {
    String tmp = "UnknownHost";

    try {
      InetAddress address = InetAddress.getLocalHost();

      try {
        tmp = address.getHostName();
      } catch (Exception e) {
        logger.error("Failed to get hostname, use address.", e);
        tmp = address.getHostAddress();
      }
    } catch (Exception e) {
      logger.error("Failed to get hostname or address.", e);
    }

    hostname = tmp;
  }

  private final ZeusClient zeusClient;
  private final ZeusSubScan zeusSubScan;
  private ZeusDB schema;


  private OperatorContext context;
  private OutputMutator output;
  private List<ProjectColumnInfo> columnInfos;
  private QueryPlan plan;
  private Iterator<RowResult> queryResult;


  private static class ProjectColumnInfo {
    ValueVector vv;
    ZeusColumnSchema zeusSchema;

    ProjectColumnInfo(ValueVector vv, ZeusColumnSchema zeusSchema) {
      this.vv = vv;
      this.zeusSchema = zeusSchema;
    }
  }

  static final Map<ColumnType, MinorType> TYPES = ImmutableMap.<ColumnType, MinorType>builder()
    .put(BOOL, MinorType.BIT)
    .put(FLOAT, MinorType.FLOAT4)
    .put(INT32, MinorType.INT)
    .put(INT64, MinorType.BIGINT)
    .put(TIMESTAMP, MinorType.TIMESTAMP)
    .put(STRING, MinorType.VARCHAR)
    .put(BYTE, MinorType.TINYINT)
    .build();

  public ZeusRecordReader(ZeusClient zeusClient,
                          ZeusSubScan zeusSubScan,
                          ZeusDB schema,
                          List<SchemaPath> projectedColumns) {
    this.zeusClient = zeusClient;
    this.zeusSubScan = zeusSubScan;
    this.schema = schema;
    setColumns(projectedColumns);
  }

  @Override
  protected Collection<SchemaPath> transformColumns(Collection<SchemaPath> projected) {
    if (isStarQuery()) {
      return schema.getTable(zeusSubScan.getTableName())
        .getAllColumnNames()
        .stream()
        .map(SchemaPath::getSimplePath)
        .collect(Collectors.toList());
    } else {
      return projected;
    }
  }

  @Override
  public void setup(OperatorContext context, OutputMutator output) throws ExecutionSetupException {
    this.context = context;
    this.output = output;

    Set<String> columnNames = this.getColumns()
      .stream()
      .map(SchemaPath::getRootSegmentPath)
      .collect(Collectors.toSet());

    context.getStats().startWait();

    try {
      columnInfos = schema.getTable(zeusSubScan.getTableName())
        .getAllColumnSchemas()
        .stream()
        .filter(f -> columnNames.contains(f.getName()))
        .map(f -> new ProjectColumnInfo(createValueVector(f), f))
        .collect(Collectors.toList());

      plan = buildQueryPlan();
      logger.info("Query plan is: {}", plan);
    } finally {
      context.getStats().stopWait();
    }
  }

  @Override
  public int next() {
    if (queryResult == null) {
      context.getStats().startWait();
      try {
        queryResult = zeusClient.query(plan)
          .getRowsList()
          .iterator();
      } finally {
        context.getStats().stopWait();
      }
    }

    int rowCount = 0;
    for (; rowCount < MAX_BATCH_COUNT && queryResult.hasNext(); rowCount++) {
      addResult(queryResult.next(), rowCount);
    }
    return rowCount;
  }

  @Override
  public void close() throws Exception {
  }

  private ValueVector createValueVector(ZeusColumnSchema zeusColumnSchema) {
    MinorType minorType = TYPES.get(zeusColumnSchema.getColumnType());
    MajorType majorType = Types.required(minorType);

    MaterializedField materializedField = MaterializedField.create(
      zeusColumnSchema.getName(), majorType);

    final Class<? extends ValueVector> clazz = TypeHelper.getValueVectorClass(
      minorType, majorType.getMode());
    ValueVector vector;
    try {
      vector = output.addField(materializedField, clazz);
      vector.allocateNew();
    } catch (SchemaChangeException e) {
      throw new RuntimeException(e);
    }

    return vector;
  }

  private QueryPlan buildQueryPlan() {
    ZeusTable table = schema.getTable(zeusSubScan.getTableName());

    List<Integer> columnIds = getColumns()
      .stream()
      .map(SchemaPath::rootName)
      .map(table::getColumnId)
      .filter(Optional::isPresent)
      .map(Optional::get)
      .collect(Collectors.toList());

    ScanNode scanNode = ScanNode.newBuilder()
      .setDbId(schema.getId())
      .setTableId(table.getId())
      .addAllColumns(columnIds)
      .build();

    PlanNode planNode = PlanNode.newBuilder()
      .setScanNode(scanNode)
      .setPlanNodeType(PlanNodeType.SCAN_NODE)
      .build();

    QueryPlan plan = QueryPlan.newBuilder()
      .setPlanId(generatePlanId())
      .setRoot(planNode)
      .build();

    return plan;
  }

  private void addResult(RowResult row, int rowIndex) {
    for (int i = 0; i < columnInfos.size(); i++) {
      ProjectColumnInfo columnInfo = columnInfos.get(i);

      switch (columnInfo.zeusSchema.getColumnType()) {
        case STRING: {
          ByteBuffer value = ByteBuffer.wrap(row.getColumns(i).getStringValue().getBytes());
          ((VarCharVector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, value, 0, value.remaining());
        }
        break;
        case BOOL: {
          ((BitVector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getBoolValue() ? 1 : 0);
        }
        break;
        case INT32: {
          ((IntVector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getI32Value());
        }
        break;
        case INT64: {
          ((BigIntVector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getI64Value());
        }
        break;
        case FLOAT: {
          ((Float4Vector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getFloatValue());
        }
        break;
        case TIMESTAMP: {
          ((TimeStampVector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getI64Value());
        }
        break;
        case BYTE: {
          ((SmallIntVector.Mutator) columnInfo.vv.getMutator())
              .setSafe(rowIndex, row.getColumns(i).getI32Value());
        }
        break;
      }
    }
  }

  private String generatePlanId() {
    return String.format("%s-%s", hostname, UUID.randomUUID().toString());
  }
}
