/**
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 * <p>
 * http://www.apache.org/licenses/LICENSE-2.0
 * <p>
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package io.github.zeus.rule;

import com.google.common.collect.Sets;
import org.apache.calcite.plan.RelOptRule;

import java.util.Set;

public class Rules {
  public static Set<RelOptRule> PHYSICAL_RULES = Sets.newHashSet(
      PushLimitToScanRule.SINGLETON,
      PushFilterToScanRule.SINGLETON,
      PushProjectToScanRule.SINGLETON
//      PushAggregateToScanRule.SINGLETON
  );

  public static Set<RelOptRule> LOGICAL_RULES = Sets.newHashSet(
      PushTopNToScanRule.SINGLETON,
      PushAggregateToScanRule.SINGLETON
  );
}
