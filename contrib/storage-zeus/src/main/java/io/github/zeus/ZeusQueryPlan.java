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

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonProperty;
import io.github.zeus.com.google.protobuf.InvalidProtocolBufferException;
import io.github.zeus.com.google.protobuf.util.JsonFormat;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.PlanNodeType;
import io.github.zeus.rpc.QueryPlan;
import io.github.zeus.rpc.ScanNode;

import java.util.List;
import java.util.Objects;
import java.util.UUID;

/**
 * A query plan wrapper.
 */
public class ZeusQueryPlan {
  private final QueryPlan plan;
  // Redundant for ser/deser
  private final String jsonPlan;


  @JsonCreator
  public ZeusQueryPlan(@JsonProperty("jsonPlan") String jsonPlan) {
    this(parseJsonPlan(jsonPlan), jsonPlan);
  }

  private ZeusQueryPlan(QueryPlan plan, String jsonPlan) {
    Objects.requireNonNull(plan, "Plan can't be null!");
    this.plan = plan;
    this.jsonPlan = jsonPlan;
  }

  @JsonProperty
  public String getJsonPlan() {
    return jsonPlan;
  }

  @JsonIgnore
  public QueryPlan getPlan() {
    return plan;
  }

  @Override
  public String toString() {
    return jsonPlan;
  }

  /**
   * Do column pruning.
   * @param columnIds
   * @return
   */
  public ZeusQueryPlan withColumnIds(List<Integer> columnIds) {
    //TODO: Check that only scan node is in the plan
    ScanNode scanNode = plan.getRoot().getScanNode();
    ScanNode newScanNode = ScanNode.newBuilder(scanNode)
        .clearColumns()
        .addAllColumns(columnIds)
        .build();

    PlanNode newPlanNode = PlanNode.newBuilder()
        .setPlanNodeType(PlanNodeType.SCAN_NODE)
        .setScanNode(newScanNode)
        .build();

    QueryPlan newQueryPlan = QueryPlan.newBuilder()
        .setPlanId(UUID.randomUUID().toString())
        .setRoot(newPlanNode)
        .build();

    return ZeusQueryPlan.from(newQueryPlan);
  }

  public ZeusQueryPlan withNewRoot(PlanNode newRoot) {
    PlanNode newRootFull = PlanNode.newBuilder(newRoot)
        .clearChildren()
        .addChildren(plan.getRoot())
        .build();

    QueryPlan newPlan = QueryPlan.newBuilder(plan)
        .setRoot(newRootFull)
        .build();

    return ZeusQueryPlan.from(newPlan);
  }

  private static QueryPlan parseJsonPlan(String jsonPlan) {
    QueryPlan.Builder builder =  QueryPlan.newBuilder();
    try {
      JsonFormat.parser().merge(jsonPlan, builder);
    } catch (InvalidProtocolBufferException e) {
      throw new RuntimeException("Failed to parse json plan.", e);
    }

    return builder.build();
  }

  private static String toJsonPlan(QueryPlan queryPlan) {
    try {
      return JsonFormat.printer()
          .includingDefaultValueFields()
          .print(queryPlan);
    } catch (InvalidProtocolBufferException e) {
      throw new RuntimeException(e);
    }
  }



  public static ZeusQueryPlan from(QueryPlan plan) {
    return new ZeusQueryPlan(plan, toJsonPlan(plan));
  }
}
