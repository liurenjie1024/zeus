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
import org.apache.drill.common.expression.SchemaPath;
import org.apache.drill.exec.planner.logical.DynamicDrillTable;
import org.apache.drill.exec.store.StoragePlugin;

import java.util.*;
import java.util.stream.Collectors;

/**
 * Created by liurenjie on 24/01/2018.
 */
public class ZeusTable extends DynamicDrillTable {
  private final ZeusDB parent;
  private final ZeusTableSchema tableSchema;

  public ZeusTable(StoragePlugin plugin, String storageEngineName, Object selection, ZeusDB parent, ZeusTableSchema tableSchema) {
    super(plugin, storageEngineName, selection);
    this.parent = parent;
    this.tableSchema = tableSchema;
  }

  public String getDBName() {
    return parent.getName();
  }

  public String getTableName() {
    return tableSchema.getName();
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

  public List<Integer> getColumnIds(List<SchemaPath> columns) {
    //TODO: Optimize this

    if (ZeusDB.isStarSchema(columns)) {
      return new ArrayList<>(tableSchema.getColumnsMap().keySet());
    }

    return columns.stream()
        .map(p -> p.getLastSegment().getNameSegment().getPath())
        .map(this::getColumnId)
        .filter(Optional::isPresent)
        .map(Optional::get)
        .collect(Collectors.toList());
  }

  public Optional<ColumnType> getColumnType(String column) {
    return getColumnId(column)
        .flatMap(columnId -> Optional.ofNullable(tableSchema.getColumnsMap().get(columnId)))
        .map(ZeusColumnSchema::getColumnType);
  }

  public Set<String> getAllColumnNames() {
    return tableSchema.getColumnsMap()
      .values()
      .stream()
      .map(ZeusColumnSchema::getName)
      .collect(Collectors.toSet());
  }

  public Collection<ZeusColumnSchema> getAllColumnSchemas() {
    return tableSchema.getColumnsMap().values();
  }


  private static RelDataType toDrillType(RelDataTypeFactory typeFactory, ColumnType columnType) {
    switch (columnType) {
      case STRING:
        return typeFactory.createSqlType(SqlTypeName.VARCHAR);
      case INT8:
        return typeFactory.createSqlType(SqlTypeName.TINYINT);
      case INT16:
        return typeFactory.createSqlType(SqlTypeName.SMALLINT);
      case INT32:
        return typeFactory.createSqlType(SqlTypeName.INTEGER);
      case INT64:
        return typeFactory.createSqlType(SqlTypeName.BIGINT);
      case BOOL:
        return typeFactory.createSqlType(SqlTypeName.BOOLEAN);
      case TIMESTAMP:
        return typeFactory.createSqlType(SqlTypeName.TIMESTAMP);
      case FLOAT4:
        return typeFactory.createSqlType(SqlTypeName.FLOAT);
      case FLOAT8:
        return typeFactory.createSqlType(SqlTypeName.DOUBLE);
      default:
        throw new IllegalArgumentException("Unrecognized type: " + columnType.name());
    }
  }
}
