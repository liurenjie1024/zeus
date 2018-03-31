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
import io.github.zeus.rpc.ColumnType;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.PlanNodeType;
import io.github.zeus.rpc.QueryPlan;
import io.github.zeus.rpc.RowResult;
import io.github.zeus.rpc.ScanNode;
import io.github.zeus.rpc.ZeusColumnSchema;
import io.github.zeus.rpc.ZeusDBSchema;
import io.github.zeus.rpc.ZeusTableSchema;
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
import org.apache.drill.exec.vector.BigIntVector;
import org.apache.drill.exec.vector.BitVector;
import org.apache.drill.exec.vector.Float4Vector;
import org.apache.drill.exec.vector.IntVector;
import org.apache.drill.exec.vector.TimeStampVector;
import org.apache.drill.exec.vector.ValueVector;
import org.apache.drill.exec.vector.VarCharVector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.ByteBuffer;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.stream.Collectors;

import static io.github.zeus.rpc.ColumnType.INT32;

public class ZeusRecordReader extends AbstractRecordReader {
  private static final Logger logger = LoggerFactory.getLogger(ZeusRecordReader.class);
  private static final int MAX_BATCH_COUNT = 4000;

  private final ZeusClient zeusClient;
  private final ZeusSubScan zeusSubScan;

  private ZeusDBSchema zeusDBSchema;
  private ZeusTableSchema zeusTableSchema;
  private OperatorContext context;
  private OutputMutator output;
  private List<ProjectColumnInfo> columnInfos;
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
    .put(ColumnType.BOOL, MinorType.BIT)
    .put(ColumnType.FLOAT, MinorType.FLOAT4)
    .put(INT32, MinorType.INT)
    .put(ColumnType.INT64, MinorType.BIGINT)
    .put(ColumnType.TIMESTAMP, MinorType.TIMESTAMP)
    .put(ColumnType.STRING, MinorType.VARBINARY)
    .build();

  public ZeusRecordReader(ZeusClient zeusClient, ZeusSubScan zeusSubScan,
                          List<SchemaPath> projectedColumns) {
    this.zeusClient = zeusClient;
    this.zeusSubScan = zeusSubScan;
    setColumns(projectedColumns);
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
      zeusDBSchema = zeusClient.getDBSchema(zeusSubScan.getDbName());
      zeusTableSchema = zeusDBSchema.getTablesMap().values()
        .stream()
        .filter(t -> t.getName().equals(zeusSubScan.getTableName()))
        .findFirst()
        .get();

      columnInfos = zeusTableSchema
        .getColumnsMap()
        .values()
        .stream()
        .filter(f -> columnNames.contains(f.getName()))
        .map(f -> new ProjectColumnInfo(createValueVector(f), f))
        .collect(Collectors.toList());
    } finally {
      context.getStats().stopWait();
    }
  }

  @Override
  public int next() {
    if (queryResult == null) {
      Set<String> columnNames = this.getColumns()
        .stream()
        .map(SchemaPath::getRootSegmentPath)
        .collect(Collectors.toSet());

      context.getStats().startWait();
      try {
        queryResult = zeusClient.query(buildQueryPlan(columnNames))
          .getRowsList().iterator();
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

  private QueryPlan buildQueryPlan(Set<String> columns) {
    List<Integer> columnIds = zeusTableSchema.getColumnsMap()
      .values()
      .stream()
      .filter(f -> columns.contains(f.getName()))
      .map(ZeusColumnSchema::getId)
      .collect(Collectors.toList());

    ScanNode scanNode = ScanNode.newBuilder()
      .setDbId(zeusDBSchema.getId())
      .setTableId(zeusTableSchema.getId())
      .addAllColumns(columnIds)
      .build();

    PlanNode planNode = PlanNode.newBuilder()
      .setScanNode(scanNode)
      .setPlanNodeType(PlanNodeType.SCAN_NODE)
      .build();

    return QueryPlan.newBuilder()
      .setPlanId(1)
      .setRoot(planNode)
      .build();
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
        case BOOL: {
          ((BitVector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getBoolValue() ? 1 : 0);
        }
        case INT32: {
          ((IntVector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getI32Value());
        }
        case INT64: {
          ((BigIntVector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getI64Value());
        }
        case FLOAT: {
          ((Float4Vector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getFloatValue());
        }
        case TIMESTAMP: {
          ((TimeStampVector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getI64Value());
        }
      }
    }
  }
}
