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

package io.github.zeus.schema;

import io.github.zeus.rpc.ColumnType;
import io.github.zeus.rpc.ZeusColumnSchema;
import io.github.zeus.rpc.ZeusTableSchema;
import org.apache.calcite.rel.type.RelDataType;
import org.apache.calcite.rel.type.RelDataTypeFactory;
import org.apache.calcite.sql.type.SqlTypeName;
import org.apache.drill.exec.planner.logical.DynamicDrillTable;
import org.apache.drill.exec.store.StoragePlugin;

import java.util.List;
import java.util.Optional;
import java.util.OptionalInt;
import java.util.Set;
import java.util.stream.Collectors;

/**
 * Created by liurenjie on 24/01/2018.
 */
public class ZeusTable extends DynamicDrillTable {
  private final ZeusTableSchema tableSchema;

  public ZeusTable(StoragePlugin plugin, String storageEngineName, Object selection, ZeusTableSchema tableSchema) {
    super(plugin, storageEngineName, selection);
    this.tableSchema = tableSchema;
  }

  @Override
  public RelDataType getRowType(final RelDataTypeFactory typeFactory) {
    List<RelDataType> dataTypes = this.tableSchema.getColumnsMap().values().stream()
      .map(f -> toDrillType(typeFactory, f.getColumnType()))
      .map(t -> typeFactory.createTypeWithNullability(t, false))
      .collect(Collectors.toList());

    List<String> names = this.tableSchema.getColumnsMap().values().stream()
      .map(ZeusColumnSchema::getName)
      .collect(Collectors.toList());

    return typeFactory.createStructType(dataTypes, names);
  }

  public int getId() {
    return tableSchema.getId();
  }

  public Optional<Integer> getColumnId(String columnName) {
    return tableSchema.getColumnsMap()
      .values()
      .stream()
      .filter(c -> c.getName().equals(columnName))
      .findFirst()
      .map(ZeusColumnSchema::getId);
  }

  public Set<String> getAllColumnNames() {
    return tableSchema.getColumnsMap()
      .values()
      .stream()
      .map(ZeusColumnSchema::getName)
      .collect(Collectors.toSet());
  }


  private static RelDataType toDrillType(RelDataTypeFactory typeFactory, ColumnType columnType) {
    switch (columnType) {
      case STRING:
        return typeFactory.createSqlType(SqlTypeName.VARCHAR);
      case FLOAT:
        return typeFactory.createSqlType(SqlTypeName.FLOAT);
      case INT32:
        return typeFactory.createSqlType(SqlTypeName.INTEGER);
      case INT64:
        return typeFactory.createSqlType(SqlTypeName.BIGINT);
      case BOOL:
        return typeFactory.createSqlType(SqlTypeName.BOOLEAN);
      case TIMESTAMP:
        return typeFactory.createSqlType(SqlTypeName.TIMESTAMP);
      case BYTE:
        return typeFactory.createSqlType(SqlTypeName.INTEGER);
      default:
        throw new IllegalArgumentException("Unrecognized type: " + columnType.name());
    }
  }
}
