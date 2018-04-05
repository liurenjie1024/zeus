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


import com.google.common.collect.ImmutableList;
import io.github.zeus.rpc.ZeusDBSchema;
import io.github.zeus.rpc.ZeusTableSchema;
import org.apache.calcite.schema.Table;
import org.apache.drill.exec.store.AbstractSchema;

import java.util.Set;
import java.util.stream.Collectors;


public class ZeusDB extends AbstractSchema {
  private final ZeusStoragePlugin plugin;
  private final String storageEngineName;
  private final ZeusDBSchema zeusDBSchema;

  public ZeusDB(ZeusStoragePlugin plugin, String storageEngineName, ZeusDBSchema zeusDBSchema) {
    super(ImmutableList.of(), zeusDBSchema.getName());
    this.plugin = plugin;
    this.storageEngineName = storageEngineName;
    this.zeusDBSchema = zeusDBSchema;
  }

  @Override
  public String getTypeName() {
    return getName();
  }

  @Override
  public Set<String> getTableNames() {
    return zeusDBSchema.getTablesMap().values()
      .stream()
      .map(ZeusTableSchema::getName)
      .collect(Collectors.toSet());
  }

  public Table getTable(String name) {
    return zeusDBSchema.getTablesMap().values()
      .stream()
      .filter(t -> t.getName().equals(name))
      .findFirst()
      .map(t -> new ZeusTable(plugin, storageEngineName, new ZeusGroupScanSpec(name), t))
      .get();
  }
}
