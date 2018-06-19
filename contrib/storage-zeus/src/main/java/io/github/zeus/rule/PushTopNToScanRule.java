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
import org.apache.calcite.rex.RexNode;
import org.apache.calcite.util.Pair;
import org.apache.drill.common.expression.LogicalExpression;
import org.apache.drill.common.logical.data.Order.Ordering;
import org.apache.drill.exec.physical.base.GroupScan;
import org.apache.drill.exec.planner.logical.DrillOptiq;
import org.apache.drill.exec.planner.logical.DrillParseContext;
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
    super(RelOptRule.operand(TopNPrel.class,
        RelOptRule.operand(ProjectPrel.class, RelOptRule.operand(ScanPrel.class, RelOptRule.none()))));
  }

  @Override
  public void onMatch(RelOptRuleCall call) {
    TopNPrel topNPrel = call.rel(0);
    ProjectPrel projectPrel = call.rel(1);
    ScanPrel scanPrel = call.rel(2);

    ZeusGroupScan zeusGroupScan = (ZeusGroupScan) scanPrel.getGroupScan();

    Optional<PlanNode> projectNode = projectPrelToPlanNode(projectPrel, call, zeusGroupScan.getTable(), scanPrel);
    Optional<PlanNode> topNNode = topnNodeToPlanNode(topNPrel, call, zeusGroupScan.getTable(), projectPrel);

    if (projectNode.isPresent() && topNNode.isPresent()) {
      ZeusGroupScan newGroupScan = zeusGroupScan.cloneWithNewRootPlanNode(projectNode.get())
          .cloneWithNewRootPlanNode(topNNode.get());
      newGroupScan.setTopNPushedDown(true);

      ScanPrel newScanPrel = ScanPrel.create(scanPrel, topNPrel.getTraitSet(), newGroupScan,
          topNPrel.getRowType());

      call.transformTo(newScanPrel);
    } else {
      ZeusGroupScan newGroupScan = zeusGroupScan.copy();
      newGroupScan.setTopNPushedDown(true);

      ScanPrel newScan = ScanPrel.create(scanPrel, scanPrel.getTraitSet(), newGroupScan, scanPrel.getRowType());
      RelNode newProject = projectPrel.copy(projectPrel.getTraitSet(), ImmutableList.of(newScan));

      call.transformTo(topNPrel.copy(topNPrel.getTraitSet(), ImmutableList.of(newProject)));
    }
  }

  @Override
  public boolean matches(RelOptRuleCall call) {
    ScanPrel scanPrel = call.rel(2);

    GroupScan groupScan = scanPrel.getGroupScan();
    if (!(groupScan instanceof ZeusGroupScan)) {
      return false;
    }

    ZeusGroupScan zeusGroupScan = (ZeusGroupScan) groupScan;
    if (zeusGroupScan.isTopNPushedDown()) {
      return false;
    }

    return super.matches(call);
  }

  private static Optional<PlanNode> projectPrelToPlanNode(ProjectPrel projectPrel, RelOptRuleCall call, ZeusTable table, RelNode input) {
    List<ProjectItem> projects = new ArrayList<>(projectPrel.getProjects().size());
    boolean allConverted = true;

    for (Pair<RexNode, String> namedProject : projectPrel.getNamedProjects()) {
      LogicalExpression logicalExpr = DrillOptiq.toDrill(
          new DrillParseContext(PrelUtil.getPlannerSettings(call.getPlanner())),
          input, namedProject.left);

      Optional<Expression> zeusExprOpt = logicalExpr.accept(
          new ZeusExprBuilder(table),
          null);

      if (!zeusExprOpt.isPresent()) {
        allConverted = false;
        break;
      }

      Expression zeusExpr = Expression.newBuilder(zeusExprOpt.get())
          .setAlias(namedProject.right)
          .build();

      ProjectItem projectItem = ProjectItem.newBuilder()
          .setAlias(namedProject.right)
          .setExpression(zeusExpr)
          .build();

      projects.add(projectItem);
    }

    if (allConverted) {
      ProjectNode projectNode = ProjectNode.newBuilder()
          .addAllItems(projects)
          .build();

      PlanNode newRoot = PlanNode.newBuilder()
          .setPlanNodeType(PlanNodeType.PROJECT_NODE)
          .setProjectNode(projectNode)
          .build();

      return Optional.of(newRoot);
    } else {
      return Optional.empty();
    }
  }

  private static Optional<PlanNode> topnNodeToPlanNode(TopNPrel topNPrel, RelOptRuleCall call, ZeusTable table, RelNode input) {
    List<Ordering> orderBys = PrelUtil.getOrdering(topNPrel.getCollation(), input.getRowType());

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
          .setLimit(topNPrel.getLimit())
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
