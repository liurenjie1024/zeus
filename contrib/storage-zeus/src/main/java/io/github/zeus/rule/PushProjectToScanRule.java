package io.github.zeus.rule;

import com.google.common.collect.ImmutableList;
import io.github.zeus.ZeusGroupScan;
import io.github.zeus.expr.ZeusExprBuilder;
import io.github.zeus.rpc.Expression;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.PlanNodeType;
import io.github.zeus.rpc.ProjectNode;
import io.github.zeus.rpc.ProjectNode.ProjectItem;
import org.apache.calcite.plan.RelOptRule;
import org.apache.calcite.plan.RelOptRuleCall;
import org.apache.calcite.rex.RexNode;
import org.apache.calcite.util.Pair;
import org.apache.drill.common.expression.LogicalExpression;
import org.apache.drill.exec.physical.base.GroupScan;
import org.apache.drill.exec.planner.logical.DrillOptiq;
import org.apache.drill.exec.planner.logical.DrillParseContext;
import org.apache.drill.exec.planner.physical.PrelUtil;
import org.apache.drill.exec.planner.physical.ProjectPrel;
import org.apache.drill.exec.planner.physical.ScanPrel;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

public class PushProjectToScanRule extends RelOptRule {
  public static final PushProjectToScanRule SINGLETON = new PushProjectToScanRule();

  private PushProjectToScanRule() {
    super(RelOptRule.operand(ProjectPrel.class,
      RelOptRule.operand(ScanPrel.class, RelOptRule.none())));
  }

  @Override
  public void onMatch(RelOptRuleCall call) {
    ProjectPrel projectPrel = call.rel(0);
    ScanPrel scanPrel = call.rel(1);

    ZeusGroupScan groupScan = (ZeusGroupScan)scanPrel.getGroupScan();

    List<ProjectItem> projects = new ArrayList<>(projectPrel.getProjects().size());
    boolean allConverted = true;

    for (Pair<RexNode, String> namedProject: projectPrel.getNamedProjects()) {
      LogicalExpression logicalExpr = DrillOptiq.toDrill(
        new DrillParseContext(PrelUtil.getPlannerSettings(call.getPlanner())),
        scanPrel, namedProject.left);

      Optional<Expression> zeusExprOpt = logicalExpr.accept(
        new ZeusExprBuilder(groupScan.getTable()),
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

      ZeusGroupScan newGroupScan = groupScan.cloneWithNewRootPlanNode(newRoot);
      newGroupScan.setProjectPushedDown(true);

      ScanPrel newScan = ScanPrel.create(
        scanPrel,
        projectPrel.getTraitSet(),
        newGroupScan,
        projectPrel.getRowType());

      call.transformTo(newScan);
    } else {
      ZeusGroupScan newGroupScan = groupScan.copy();
      newGroupScan.setFilterPushedDown(true);

      ScanPrel newScan = ScanPrel.create(scanPrel, scanPrel.getTraitSet(), newGroupScan, scanPrel.getRowType());

      call.transformTo(projectPrel.copy(projectPrel.getTraitSet(), ImmutableList.of(newScan)));
    }


  }

  @Override
  public boolean matches(RelOptRuleCall call) {
    ScanPrel scanPrel = call.rel(1);
    GroupScan groupScan = scanPrel.getGroupScan();

    if (!(groupScan instanceof ZeusGroupScan)) {
      return false;
    }

    ZeusGroupScan zeusGroupScan = (ZeusGroupScan) groupScan;
    if (zeusGroupScan.isProjectPushedDown()) {
      return false;
    }

    return super.matches(call);
  }
}
