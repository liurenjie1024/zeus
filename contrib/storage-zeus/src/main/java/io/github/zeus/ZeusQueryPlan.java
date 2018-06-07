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
import io.github.zeus.rpc.QueryPlan;

import java.util.Objects;

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
