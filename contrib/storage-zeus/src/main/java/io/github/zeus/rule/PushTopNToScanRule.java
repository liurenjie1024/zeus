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

import com.google.common.collect.ImmutableList;
import io.github.zeus.ZeusGroupScan;
import io.github.zeus.expr.ZeusExprBuilder;
import io.github.zeus.rpc.Expression;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.PlanNodeType;
import io.github.zeus.rpc.ProjectNode;
import io.github.zeus.rpc.ProjectNode.ProjectItem;
import io.github.zeus.rpc.TopNNode;
import io.github.zeus.rpc.TopNNode.SortItem;
import io.github.zeus.schema.ZeusTable;
import org.apache.calcite.plan.RelOptRule;
import org.apache.calcite.plan.RelOptRuleCall;
import org.apache.calcite.rel.RelNode;
import org.apache.calcite.rex.RexLiteral;
import org.apache.calcite.rex.RexNode;
import org.apache.calcite.util.Pair;
import org.apache.drill.common.expression.LogicalExpression;
import org.apache.drill.common.logical.data.Order.Ordering;
import org.apache.drill.exec.physical.base.GroupScan;
import org.apache.drill.exec.planner.logical.DrillLimitRel;
import org.apache.drill.exec.planner.logical.DrillOptiq;
import org.apache.drill.exec.planner.logical.DrillParseContext;
import org.apache.drill.exec.planner.logical.DrillScanRel;
import org.apache.drill.exec.planner.logical.DrillSortRel;
import org.apache.drill.exec.planner.physical.PrelUtil;
import org.apache.drill.exec.planner.physical.ProjectPrel;
import org.apache.drill.exec.planner.physical.ScanPrel;
import org.apache.drill.exec.planner.physical.TopNPrel;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

public class PushTopNToScanRule extends RelOptRule {
  public static final PushTopNToScanRule SINGLETON = new PushTopNToScanRule();

  private PushTopNToScanRule() {
    super(RelOptRule.operand(DrillLimitRel.class,
        RelOptRule.operand(DrillSortRel.class, RelOptRule.operand(DrillScanRel.class, RelOptRule.none()))));
  }

  @Override
  public void onMatch(RelOptRuleCall call) {
    DrillLimitRel limitDrel = call.rel(0);
    DrillSortRel sortDrel = call.rel(1);
    DrillScanRel scanDrel = call.rel(2);

    ZeusGroupScan zeusGroupScan = (ZeusGroupScan) scanDrel.getGroupScan();

    Optional<PlanNode> topNNode = topnNodeToPlanNode(limitDrel, sortDrel, call, zeusGroupScan.getTable(), scanDrel);

    if (topNNode.isPresent()) {
      ZeusGroupScan newGroupScan = zeusGroupScan.cloneWithNewRootPlanNode(topNNode.get());
      newGroupScan.setTopNPushedDown(true);

      DrillScanRel newScanDrel = new DrillScanRel(scanDrel.getCluster(),
          scanDrel.getTraitSet(), scanDrel.getTable(), newGroupScan, scanDrel.getRowType(),
          scanDrel.getColumns());

      RelNode newSortDrel = sortDrel.copy(sortDrel.getTraitSet(), ImmutableList.of(newScanDrel));
      call.transformTo(limitDrel.copy(limitDrel.getTraitSet(), ImmutableList.of(newSortDrel)));
    } else {
      ZeusGroupScan newGroupScan = zeusGroupScan.copy();
      newGroupScan.setTopNPushedDown(true);

      DrillScanRel newScanDrel = new DrillScanRel(scanDrel.getCluster(),
          scanDrel.getTraitSet(), scanDrel.getTable(), newGroupScan, scanDrel.getRowType(),
          scanDrel.getColumns());

      RelNode newSortDrel = sortDrel.copy(sortDrel.getTraitSet(), ImmutableList.of(newScanDrel));
      call.transformTo(limitDrel.copy(limitDrel.getTraitSet(), ImmutableList.of(newSortDrel)));
    }
  }

  @Override
  public boolean matches(RelOptRuleCall call) {
    DrillScanRel scanDrel = call.rel(2);

    GroupScan groupScan = scanDrel.getGroupScan();
    if (!(groupScan instanceof ZeusGroupScan)) {
      return false;
    }

    ZeusGroupScan zeusGroupScan = (ZeusGroupScan) groupScan;
    if (zeusGroupScan.isTopNPushedDown()) {
      return false;
    }

    DrillLimitRel limitRel = call.rel(0);
    if (limitRel.getOffset() != null) {
      return false;
    }

    return super.matches(call);
  }

  private static Optional<PlanNode> topnNodeToPlanNode(DrillLimitRel limitRel, DrillSortRel sortRel, RelOptRuleCall call, ZeusTable table, RelNode input) {
    List<Ordering> orderBys = PrelUtil.getOrdering(sortRel.getCollation(), input.getRowType());

    List<SortItem> sortItems = new ArrayList<>(orderBys.size());
    boolean allConverted = true;
    for (Ordering ordering : orderBys) {
      Optional<Expression> exprOpt = ordering.getExpr().accept(
          new ZeusExprBuilder(table), null);

      if (!exprOpt.isPresent()) {
        allConverted = false;
        break;
      }

      SortItem sortItem = SortItem.newBuilder()
          .setExpr(exprOpt.get())
          .setDesc(ordering.getDirection().isDescending())
          .build();

      sortItems.add(sortItem);
    }

    if (allConverted) {
      TopNNode topNNode = TopNNode.newBuilder()
          .setLimit(RexLiteral.intValue(limitRel.getFetch()))
          .addAllSortItem(sortItems)
          .build();

      PlanNode newRoot = PlanNode.newBuilder()
          .setPlanNodeType(PlanNodeType.TOPN_NODE)
          .setTopnNode(topNNode)
          .build();

      return Optional.of(newRoot);
    } else {
      return Optional.empty();
    }
  }
}
