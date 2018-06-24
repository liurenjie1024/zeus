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

import com.google.common.collect.ImmutableList;
import io.github.zeus.ZeusGroupScan;
import io.github.zeus.expr.ZeusExprBuilder;
import io.github.zeus.rpc.Expression;
import io.github.zeus.rpc.FilterNode;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.PlanNodeType;
import org.apache.calcite.plan.RelOptRule;
import org.apache.calcite.plan.RelOptRuleCall;
import org.apache.calcite.rex.RexNode;
import org.apache.drill.common.expression.LogicalExpression;
import org.apache.drill.exec.planner.logical.DrillFilterRel;
import org.apache.drill.exec.planner.logical.DrillOptiq;
import org.apache.drill.exec.planner.logical.DrillParseContext;
import org.apache.drill.exec.planner.logical.DrillScanRel;
import org.apache.drill.exec.planner.physical.FilterPrel;
import org.apache.drill.exec.planner.physical.PrelUtil;
import org.apache.drill.exec.planner.physical.ScanPrel;

import java.util.Optional;

public class PushFilterToScanRule extends RelOptRule {
  public static final PushFilterToScanRule SINGLETON = new PushFilterToScanRule();

  private PushFilterToScanRule() {
    super(RelOptRule.operand(DrillFilterRel.class, RelOptRule.operand(DrillScanRel.class, RelOptRule
      .none())));
  }

  @Override
  public void onMatch(RelOptRuleCall call) {
    DrillFilterRel filterRel = call.rel(0);
    DrillScanRel scanRel = call.rel(1);

    RexNode condition = filterRel.getCondition();

    final LogicalExpression conditionExp = DrillOptiq.toDrill(
        new DrillParseContext(PrelUtil.getPlannerSettings(call.getPlanner())),
        scanRel, condition);

    ZeusGroupScan groupScan = (ZeusGroupScan) scanRel.getGroupScan();
    ZeusExprBuilder builder = new ZeusExprBuilder(groupScan.getTable());
    Optional<Expression> zeusExpr = conditionExp.accept(builder, null);

    if (zeusExpr.isPresent()) {
      FilterNode filterNode = FilterNode.newBuilder()
          .addConditions(zeusExpr.get())
          .build();

      PlanNode filterPlanNode = PlanNode.newBuilder()
          .setPlanNodeType(PlanNodeType.FILTER_NODE)
          .setFilterNode(filterNode)
          .build();

      ZeusGroupScan newGroupScan = groupScan.cloneWithNewRootPlanNode(filterPlanNode);
      newGroupScan.setFilterPushedDown(true);

      DrillScanRel newScan = new DrillScanRel(
        scanRel.getCluster(),
        filterRel.getTraitSet(),
        scanRel.getTable(),
        newGroupScan,
        filterRel.getRowType(),
        scanRel.getColumns(),
        false);
      call.transformTo(newScan);

    } else {
      ZeusGroupScan newGroupScan = groupScan.copy();
      newGroupScan.setFilterPushedDown(true);


      DrillScanRel newScan = new DrillScanRel(
        scanRel.getCluster(),
        filterRel.getTraitSet(),
        scanRel.getTable(),
        newGroupScan,
        filterRel.getRowType(),
        scanRel.getColumns(),
        false);

      call.transformTo(filterRel.copy(filterRel.getTraitSet(), ImmutableList.of(newScan)));
    }
  }


  @Override
  public boolean matches(RelOptRuleCall call) {
    DrillScanRel scanPrel = call.rel(1);

    if (!(scanPrel.getGroupScan() instanceof ZeusGroupScan)) {
      return false;
    }

    ZeusGroupScan groupScan = (ZeusGroupScan) scanPrel.getGroupScan();
    if (groupScan.isFilterPushedDown()) {
      return false;
    }

    return super.matches(call);
  }
}
