package io.github.zeus.rule;

import io.github.zeus.ZeusTableScan;
import io.github.zeus.rpc.PlanNode;
import org.apache.calcite.plan.RelOptRule;
import org.apache.calcite.plan.RelOptRuleCall;
import org.apache.drill.exec.planner.physical.ScanPrel;

public class ScanTransformRule extends RelOptRule {
  ScanTransformRule() {
    super(RelOptRule.operand(ScanPrel.class, RelOptRule.none()));
  }

  @Override
  public void onMatch(RelOptRuleCall call) {
    ScanPrel scanPrel = call.rel(0);
    PlanNode plan = ((ZeusTableScan) scanPrel.getGroupScan()).toPlanNode();

    call.transformTo(ZeusScanPrel.from(scanPrel, plan));
  }
}
