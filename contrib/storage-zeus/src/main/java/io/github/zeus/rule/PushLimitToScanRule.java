/**
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

package io.github.zeus.rule;

import io.github.zeus.ZeusGroupScan;
import io.github.zeus.rpc.LimitNode;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.PlanNodeType;
import org.apache.calcite.plan.RelOptRule;
import org.apache.calcite.plan.RelOptRuleCall;
import org.apache.calcite.rex.RexLiteral;
import org.apache.drill.exec.planner.physical.LimitPrel;
import org.apache.drill.exec.planner.physical.ScanPrel;

public class PushLimitToScanRule extends RelOptRule {
  static final PushLimitToScanRule SINGLETON = new PushLimitToScanRule();

  private PushLimitToScanRule() {
    super(RelOptRule.operand(LimitPrel.class, RelOptRule.operand(ScanPrel.class, RelOptRule.none())));
  }

  @Override
  public void onMatch(RelOptRuleCall call) {
    LimitPrel limitPrel = call.rel(0);
    ScanPrel scanPrel = call.rel(1);
    int limit = RexLiteral.intValue(limitPrel.getFetch());

    LimitNode limitNode = LimitNode.newBuilder()
        .setLimit(limit)
        .build();
    PlanNode planNode = PlanNode.newBuilder()
        .setPlanNodeType(PlanNodeType.LIMIT_NODE)
        .setLimitNode(limitNode)
        .build();

    ZeusGroupScan groupScan = (ZeusGroupScan) scanPrel.getGroupScan();

    ScanPrel newScan = ScanPrel.create(scanPrel,
        limitPrel.getTraitSet(),
        groupScan.cloneWithNewRootPlanNode(planNode),
        limitPrel.getRowType());

    call.transformTo(newScan);
  }

  @Override
  public boolean matches(RelOptRuleCall call) {
    ScanPrel scanPrel = call.rel(1);

    if (!(scanPrel.getGroupScan() instanceof ZeusGroupScan)) {
      return false;
    }

    LimitPrel limitPrel = call.rel(0);
    if (limitPrel.getOffset() != null) {
      return false;
    }

    return super.matches(call);
  }
}
