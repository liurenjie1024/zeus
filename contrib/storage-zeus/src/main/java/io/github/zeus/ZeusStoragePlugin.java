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

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.github.zeus.client.ZeusClient;
import io.github.zeus.rule.Rules;
import io.github.zeus.schema.ZeusDB;
import org.apache.calcite.plan.RelOptRule;
import org.apache.calcite.schema.SchemaPlus;
import org.apache.drill.common.JSONOptions;
import org.apache.drill.common.expression.SchemaPath;
import org.apache.drill.common.logical.StoragePluginConfig;
import org.apache.drill.exec.ops.OptimizerRulesContext;
import org.apache.drill.exec.planner.PlannerPhase;
import org.apache.drill.exec.server.DrillbitContext;
import org.apache.drill.exec.store.AbstractStoragePlugin;
import org.apache.drill.exec.store.SchemaConfig;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Collections;
import java.util.List;
import java.util.Set;

public class ZeusStoragePlugin extends AbstractStoragePlugin {
  private static final Logger logger = LoggerFactory.getLogger(ZeusStoragePlugin.class);

  private final ZeusStoragePluginConfig config;
  private final DrillbitContext context;
  private final String name;
  private ZeusClient client;
  private ZeusDB dbSchema;

  public ZeusStoragePlugin(ZeusStoragePluginConfig config, DrillbitContext context, String name) {
    this.config = config;
    this.context = context;
    this.name = name;

    logger.info("Zeus storage plugin created, name: {}, config: {}", name, config);
  }


  @Override
  public void registerSchemas(SchemaConfig schemaConfig, SchemaPlus parent) throws IOException {
    parent.add(name, dbSchema);
  }

  @Override
  public StoragePluginConfig getConfig() {
    return config;
  }

  @Override
  public Set<? extends RelOptRule> getOptimizerRules(OptimizerRulesContext optimizerContext, PlannerPhase phase) {
    switch (phase) {
      case PHYSICAL:
        return Rules.PHYSICAL_RULES;
      default:
        return Collections.emptySet();
    }
  }

  @Override
  public void start() throws IOException {
    logger.info("Starting zeus storage plugin");
    try {
      client = config.getClient();
      dbSchema = new ZeusDB(this, name, client.getDBSchema(name).get());
    } catch (Throwable t) {
      logger.error("Failed to start zeus plugin.", t);
      throw t;
    }
    logger.info("Succeeded to start zeus plugin.");
  }

  public ZeusClient getClient() {
    return client;
  }

  @Override
  public boolean supportsRead() {
    return true;
  }

  @Override
  public boolean supportsWrite() {
    return false;
  }

  @Override
  public ZeusTableScan getPhysicalScan(String userName,
                                       JSONOptions selection,
                                       List<SchemaPath> paths) throws IOException {
    Integer tableId = selection.getListWith(new ObjectMapper(),
        new TypeReference<Integer>() {});
    return ZeusTableScan.from(this, dbSchema.getId(), tableId, paths);
  }

  ZeusDB getDbSchema() {
    return dbSchema;
  }

  @Override
  public void close() {
    if (client != null) {
      try {
        client.close();
        logger.info("Zeus plugin stopped.");
      } catch (Throwable t) {
        logger.error("Failed to close zeus client.", t);
      }
    }
  }
}

