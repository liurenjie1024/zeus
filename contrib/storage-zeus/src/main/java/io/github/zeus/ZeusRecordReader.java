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
import io.github.zeus.client.meta.ColumnMeta;
import io.github.zeus.rpc.*;
import org.apache.drill.common.exceptions.ExecutionSetupException;
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
  private final ZeusQueryPlan queryPlan;


  private OperatorContext context;
  private OutputMutator output;
  private List<ProjectColumnInfo> columnInfos;
  private Iterator<RowResult> queryResult;


  private static class ProjectColumnInfo {
    ValueVector vv;
    ColumnMeta columnMeta;

    ProjectColumnInfo(ValueVector vv, ColumnMeta columnMeta) {
      this.vv = vv;
      this.columnMeta = columnMeta;
    }
  }

  static final Map<ColumnType, MinorType> TYPES = ImmutableMap.<ColumnType, MinorType>builder()
    .put(BOOL, MinorType.BIT)
    .put(INT8, MinorType.INT)
    .put(INT16, MinorType.INT)
    .put(INT32, MinorType.INT)
    .put(INT64, MinorType.BIGINT)
    .put(FLOAT4, MinorType.FLOAT4)
    .put(FLOAT8, MinorType.FLOAT8)
    .put(TIMESTAMP, MinorType.TIMESTAMP)
    .put(STRING, MinorType.VARCHAR)
    .build();

  public ZeusRecordReader(ZeusClient zeusClient,
                          ZeusQueryPlan queryPlan) {
    this.zeusClient = zeusClient;
    this.queryPlan  = queryPlan;
  }


  @Override
  public void setup(OperatorContext context, OutputMutator output) {
    this.context = context;
    this.output = output;

    context.getStats().startWait();

    try {
      columnInfos = zeusClient.getResultMeta(queryPlan.getPlan())
          .getColumnMetaList()
          .stream()
          .map(c -> new ProjectColumnInfo(createValueVector(c), c))
          .collect(Collectors.toList());

      logger.info("Query plan is: {}", queryPlan.getJsonPlan());
    } finally {
      context.getStats().stopWait();
    }
  }

  @Override
  public int next() {
    if (queryResult == null) {
      context.getStats().startWait();
      try {
        queryResult = zeusClient.query(queryPlan.getPlan())
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

    logger.info("Row count from server is:{}", rowCount);
    return rowCount;
  }

  @Override
  public void close() throws Exception {
  }

  private ValueVector createValueVector(ColumnMeta columnMeta) {
    MinorType minorType = TYPES.get(columnMeta.getColumnType());
    MajorType majorType = Types.required(minorType);

    MaterializedField materializedField = MaterializedField.create(
      columnMeta.getColumnName(), majorType);

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

  private void addResult(RowResult row, int rowIndex) {
    for (int i = 0; i < columnInfos.size(); i++) {
      ProjectColumnInfo columnInfo = columnInfos.get(i);

      switch (columnInfo.columnMeta.getColumnType()) {
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
        case INT8:
        case INT16:
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
        case FLOAT4: {
          ((Float4Vector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getFloatValue());
        }
        break;
        case FLOAT8: {
          ((Float8Vector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getDoubleValue());
        }
        break;
        case TIMESTAMP: {
          ((TimeStampVector.Mutator) columnInfo.vv.getMutator())
            .setSafe(rowIndex, row.getColumns(i).getI64Value());
        }
        break;
      }
    }
  }
}
