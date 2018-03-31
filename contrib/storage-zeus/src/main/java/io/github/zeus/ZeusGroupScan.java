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

import com.fasterxml.jackson.annotation.JacksonInject;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonTypeName;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.drill.common.JSONOptions;
import org.apache.drill.common.exceptions.ExecutionSetupException;
import org.apache.drill.common.expression.SchemaPath;
import org.apache.drill.common.logical.StoragePluginConfig;
import org.apache.drill.exec.physical.PhysicalOperatorSetupException;
import org.apache.drill.exec.physical.base.AbstractGroupScan;
import org.apache.drill.exec.physical.base.PhysicalOperator;
import org.apache.drill.exec.physical.base.SubScan;
import org.apache.drill.exec.proto.CoordinationProtos.DrillbitEndpoint;
import org.apache.drill.exec.store.StoragePluginRegistry;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.List;

@JsonTypeName("zeus-scan")
public class ZeusGroupScan extends AbstractGroupScan {
  private static final Logger logger = LoggerFactory.getLogger(ZeusGroupScan.class);

  private final String dbName;
  private final String tableName;
  private final List<SchemaPath> columns;
  private final ZeusStoragePluginConfig config;
  private final ZeusStoragePlugin plugin;

  @JsonCreator
  public ZeusGroupScan(@JsonProperty("dbName") String dbName,
                     @JsonProperty("tableName") String tableName,
                     @JsonProperty("columns") List<SchemaPath> columns,
                     @JsonProperty("config")StoragePluginConfig config,
                     @JacksonInject StoragePluginRegistry registry) throws ExecutionSetupException {
    this(dbName, tableName, columns, (ZeusStoragePluginConfig)config,
      (ZeusStoragePlugin) registry.getPlugin(config));
  }

  ZeusGroupScan(String dbName, String tableName, List<SchemaPath> columns,
              ZeusStoragePluginConfig config, ZeusStoragePlugin plugin) {
    super("");
    this.dbName = dbName;
    this.tableName = tableName;
    this.columns = columns;
    this.config = config;
    this.plugin = plugin;
  }


  @Override
  public void applyAssignments(List<DrillbitEndpoint> endpoints) throws PhysicalOperatorSetupException {

  }

  @Override
  public SubScan getSpecificScan(int minorFragmentId) throws ExecutionSetupException {
    return new ZeusSubScan(dbName, tableName, columns, config, plugin);
  }

  @Override
  public int getMaxParallelizationWidth() {
    return 1;
  }

  @JsonProperty
  public String getDbName() {
    return dbName;
  }

  @JsonProperty
  public String getTableName() {
    return tableName;
  }

  @JsonProperty
  public List<SchemaPath> getColumns() {
    return columns;
  }

  @JsonProperty
  public ZeusStoragePluginConfig getConfig() {
    return config;
  }

  @Override
  public ZeusGroupScan clone(List<SchemaPath> columns) {
    return new ZeusGroupScan(dbName, tableName, columns, config, plugin);
  }

  @Override
  public String getDigest() {
    return String.format("ZeusGroupScan[db=%s,table=%s,columns=%s]", dbName, tableName, columns);
  }

  @Override
  public PhysicalOperator getNewWithChildren(List<PhysicalOperator> children) throws ExecutionSetupException {
    return this;
  }
}
