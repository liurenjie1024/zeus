package io.github.zeus.rule;

import org.apache.calcite.plan.RelOptRule;
import org.apache.calcite.plan.RelOptRuleCall;
import org.apache.drill.common.logical.data.Project;
import org.apache.drill.exec.planner.physical.ProjectPrel;
import org.apache.drill.exec.planner.physical.ScanPrel;
import org.apache.drill.exec.store.StoragePluginOptimizerRule;

public class ProjectPushdownRule extends StoragePluginOptimizerRule {
  private ProjectPushdownRule() {
    super(RelOptRule.operand(ProjectPrel.class, RelOptRule.operand(ScanPrel.class, RelOptRule.none())),
        "Zeus project push down rule");
  }

  @Override
  public void onMatch(RelOptRuleCall call) {
    ProjectPrel projectPrel = call.rel(0);
    ScanPrel scanPrel = call.rel(1);


  }
}
