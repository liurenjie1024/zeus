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


import com.google.common.collect.ImmutableList;
import io.github.zeus.ZeusStoragePlugin;
import static io.github.zeus.client.exception.CatalogNotFoundException.columnNotFound;

import io.github.zeus.client.exception.CatalogNotFoundException;
import io.github.zeus.rpc.*;
import org.apache.drill.common.expression.SchemaPath;
import org.apache.drill.exec.store.AbstractSchema;

import java.util.*;
import java.util.stream.Collectors;


public class ZeusDB extends AbstractSchema {
  private final ZeusStoragePlugin plugin;
  private final String storageEngineName;
  private final ZeusDBSchema dbSchema;

  public ZeusDB(ZeusStoragePlugin plugin, String storageEngineName, ZeusDBSchema dbSchema) {
    super(ImmutableList.of(), dbSchema.getName());
    this.plugin = plugin;
    this.storageEngineName = storageEngineName;
    this.dbSchema = dbSchema;
  }

  @Override
  public String getTypeName() {
    return getName();
  }

  @Override
  public Set<String> getTableNames() {
    return dbSchema.getTablesMap().values()
      .stream()
      .map(ZeusTableSchema::getName)
      .collect(Collectors.toSet());
  }

  @Override
  public ZeusTable getTable(String name) {
    return getTableSchema(name)
        .map(schema -> new ZeusTable(plugin, storageEngineName, schema.getId(), schema))
        .orElse(null);
  }

  public int getId() {
    return dbSchema.getId();
  }

  public Optional<ZeusTable> getTable(int tableId) {
    return Optional.ofNullable(dbSchema.getTablesMap().get(tableId))
        .map(schema -> new ZeusTable(plugin, storageEngineName, schema.getName(), schema));
  }

  public QueryPlan getTableScanQueryPlan(int tableId, List<SchemaPath> columns) {
    boolean isStarQuery = columns.stream().anyMatch(path -> path.equals(SchemaPath.STAR_COLUMN));

    if (isStarQuery) {
      return buildTableScanPlan(tableId, null, true);
    } else {
      List<String> columnNames = columns.stream()
          .map(p -> p.getLastSegment().getNameSegment().getPath())
          .collect(Collectors.toList());

      return buildTableScanPlan(tableId, columnNames, false);
    }
  }

  private Optional<ZeusTableSchema> getTableSchema(String tableName) {
    return dbSchema.getTablesMap().values()
        .stream()
        .filter(t -> t.getName().equals(tableName))
        .findFirst();
  }

  private QueryPlan buildTableScanPlan(int tableId, List<String> columnNames, boolean isStarQuery) {
    return Optional.ofNullable(dbSchema.getTablesMap().get(tableId))
        .map(t -> buildTableScanNode(t, columnNames, isStarQuery))
        .orElseThrow(() -> CatalogNotFoundException.tableIdNotFound(dbSchema.getId(), tableId));
  }

  private QueryPlan buildTableScanNode(ZeusTableSchema tableSchema, List<String> columnNames, boolean isStarQuery) {
    Collection<Integer> columnIds;

    if (isStarQuery) {
      columnIds = tableSchema.getColumnsMap().keySet();
    } else {
      Map<String, Integer> columnNameToId = new HashMap<>(tableSchema.getColumnsCount());
      tableSchema.getColumnsMap().values()
          .forEach(c -> columnNameToId.put(c.getName(), c.getId()));

      columnNames.stream()
          .filter(name -> !columnNameToId.containsKey(name))
          .findFirst()
          .ifPresent(name -> {
            throw columnNotFound(dbSchema.getName(), tableSchema.getName(), name);
          });

      columnIds = columnNames
          .stream()
          .map(columnNameToId::get)
          .collect(Collectors.toList());
    }



    ScanNode scanNode = ScanNode.newBuilder()
        .setDbId(dbSchema.getId())
        .setTableId(tableSchema.getId())
        .addAllColumns(columnIds)
        .build();

    PlanNode planNode = PlanNode.newBuilder()
        .setPlanNodeType(PlanNodeType.SCAN_NODE)
        .setScanNode(scanNode)
        .build();

    return QueryPlan.newBuilder()
        .setPlanId(UUID.randomUUID().toString())
        .setRoot(planNode)
        .build();
  }

  public static List<String> toColumnNames(List<SchemaPath> paths) {
    return paths.stream()
        .map(p -> p.getLastSegment().getNameSegment().getPath())
        .collect(Collectors.toList());
  }

  public static boolean isStarSchema(List<SchemaPath> columns) {
    return columns.stream().anyMatch(path -> path.equals(SchemaPath.STAR_COLUMN));
  }
}
