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
import io.github.zeus.client.exception.CatalogNotFoundException;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.schema.ZeusTable;
import org.apache.drill.common.exceptions.ExecutionSetupException;
import org.apache.drill.common.expression.SchemaPath;
import org.apache.drill.common.logical.StoragePluginConfig;
import org.apache.drill.exec.physical.PhysicalOperatorSetupException;
import org.apache.drill.exec.physical.base.AbstractGroupScan;
import org.apache.drill.exec.physical.base.PhysicalOperator;
import org.apache.drill.exec.physical.base.ScanStats;
import org.apache.drill.exec.physical.base.SubScan;
import org.apache.drill.exec.proto.CoordinationProtos.DrillbitEndpoint;
import org.apache.drill.exec.store.StoragePluginRegistry;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;

@JsonTypeName("zeus-scan")
public class ZeusGroupScan extends AbstractGroupScan {
  private static final Logger logger = LoggerFactory.getLogger(ZeusGroupScan.class);

  private final int tableId;
  private final ZeusQueryPlan queryPlan;
  private final ZeusStoragePluginConfig config;
  private final ZeusStoragePlugin plugin;
  private boolean isFilterPushedDown;

  @JsonCreator
  public ZeusGroupScan(
      @JsonProperty("tableId") int tableId,
      @JsonProperty("queryPlan") ZeusQueryPlan queryPlan,
      @JsonProperty("config") StoragePluginConfig config,
      @JsonProperty("isFilterPushedDown") boolean isFilterPushedDown,
      @JacksonInject StoragePluginRegistry registry) throws ExecutionSetupException {
    this(tableId, queryPlan, (ZeusStoragePluginConfig)config,
      (ZeusStoragePlugin) registry.getPlugin(config), isFilterPushedDown);
  }

  ZeusGroupScan(
      int tableId,
      ZeusQueryPlan queryPlan,
      ZeusStoragePluginConfig config,
      ZeusStoragePlugin plugin,
      boolean isFilterPushedDown) {
    super("");
    this.tableId = tableId;
    this.queryPlan = queryPlan;
    this.config = config;
    this.plugin = plugin;
    this.isFilterPushedDown = isFilterPushedDown;
  }


  @Override
  public void applyAssignments(List<DrillbitEndpoint> endpoints) throws PhysicalOperatorSetupException {
  }

  @Override
  public SubScan getSpecificScan(int minorFragmentId) throws ExecutionSetupException {
    return new ZeusSubScan(queryPlan, config, plugin);
  }

  @Override
  public ScanStats getScanStats() {
    return ScanStats.TRIVIAL_TABLE;
  }

  @Override
  public int getMaxParallelizationWidth() {
    return 1;
  }

  @JsonProperty
  public ZeusStoragePluginConfig getConfig() {
    return config;
  }

  @JsonProperty
  public boolean isFilterPushedDown() {
    return isFilterPushedDown;
  }

  @Override
  public ZeusGroupScan clone(List<SchemaPath> columns) {
    List<Integer> columnIds = plugin.getDbSchema()
        .getTable(tableId)
        .map(t -> t.getColumnIds(columns))
        .orElseThrow(() -> CatalogNotFoundException.tableIdNotFound(plugin.getDbSchema().getId(), tableId));

    ZeusQueryPlan newPlan = queryPlan.withColumnIds(columnIds);

    return new ZeusGroupScan(tableId, newPlan, config, plugin, isFilterPushedDown);
  }

  @Override
  public boolean canPushdownProjects(List<SchemaPath> columns) {
    return true;
  }

  public ZeusGroupScan cloneWithNewRootPlanNode(PlanNode newRoot) {
    return new ZeusGroupScan(tableId, queryPlan.withNewRoot(newRoot), config, plugin, isFilterPushedDown);
  }

  public ZeusGroupScan copy() {
    return new ZeusGroupScan(tableId, queryPlan, config, plugin, isFilterPushedDown);
  }

  public void setFilterPushedDown(boolean isFilterPushedDown) {
    this.isFilterPushedDown= isFilterPushedDown;
  }

  @Override
  public String getDigest() {
    return String.format("ZeusGroupScan[plan=%s]", queryPlan);
  }

  @Override
  public PhysicalOperator getNewWithChildren(List<PhysicalOperator> children) throws ExecutionSetupException {
    return this;
  }

  public ZeusTable getTable() {
    return plugin.getDbSchema().getTable(tableId).get();
  }
}
