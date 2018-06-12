package io.github.zeus.rule;

import io.github.zeus.rpc.LimitNode;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.PlanNodeType;
import org.apache.calcite.plan.RelOptRule;
import org.apache.calcite.plan.RelOptRuleCall;
import org.apache.calcite.rex.RexLiteral;
import org.apache.drill.exec.planner.physical.LimitPrel;

public class LimitTransformRule extends RelOptRule {
  private static final LimitTransformRule SINGLETON = new LimitTransformRule();

  public static LimitTransformRule getSingleton() {
    return SINGLETON;
  }

  private LimitTransformRule() {
    super(RelOptRule.operand(LimitPrel.class, RelOptRule.any()));
  }

  @Override
  public void onMatch(RelOptRuleCall call) {
    LimitPrel limitPrel = call.rel(0);

    int limit = RexLiteral.intValue(limitPrel.getOffset());

    LimitNode limitNode = LimitNode.newBuilder()
        .setLimit(limit)
        .build();

    PlanNode planNode = PlanNode.newBuilder()
        .setPlanNodeType(PlanNodeType.LIMIT_NODE)
        .setLimitNode(limitNode)
        .build();

    call.transformTo(ZeusInterimPrel.from(limitPrel, planNode));
  }
}
