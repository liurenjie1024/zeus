package io.github.zeus.rule;

import com.beust.jcommander.internal.Lists;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.QueryPlan;
import org.apache.calcite.plan.RelOptCluster;
import org.apache.calcite.plan.RelTraitSet;
import org.apache.calcite.rel.AbstractRelNode;
import org.apache.calcite.rel.RelNode;
import org.apache.calcite.rel.SingleRel;
import org.apache.drill.exec.planner.physical.Prel;
import org.apache.drill.exec.planner.sql.handlers.PrelFinalizable;

import java.util.Collections;
import java.util.List;
import java.util.UUID;

public class ZeusInterimPrel extends AbstractRelNode implements PrelFinalizable {
  // Nullable
  private final RelNode child;
  private final PlanNode planNode;

  public ZeusInterimPrel(RelOptCluster cluster, RelTraitSet traits, RelNode child, PlanNode planNode) {
    super(cluster, traits);
    this.child = child;
    this.planNode = planNode;
  }

  @Override
  public Prel finalizeRel() {
    QueryPlan queryPlan = QueryPlan.newBuilder()
        .setPlanId(UUID.randomUUID().toString())
        .setRoot(toQueryPlanNode())
        .build();

    return new ZeusPrel(getCluster(), getTraitSet(), queryPlan);
  }

  @Override
  public RelNode copy(RelTraitSet traitSet, List<RelNode> inputs) {
    return new ZeusInterimPrel(getCluster(), traitSet, inputs.get(0));
  }

  @Override
  protected Object clone() throws CloneNotSupportedException {
    return copy(getTraitSet(), getInputs());
  }

  @Override
  public List<RelNode> getInputs() {
    if (child == null) {
      return Collections.emptyList();
    } else {
      return Lists.newArrayList(child);
    }
  }

  public PlanNode toQueryPlanNode() {
    if (child != null) {
      ZeusInterimPrel zeusChild = (ZeusInterimPrel)child;
      return PlanNode.newBuilder(planNode)
          .clearChildren()
          .addChildren(zeusChild.planNode)
          .build();
    } else {
      return planNode;
    }
  }

  public static ZeusInterimPrel fromInterimNode(SingleRel origin, PlanNode planNode) {
    return new ZeusInterimPrel(origin.getCluster(), origin.getTraitSet(), origin.getInput(), planNode);
  }

  public static ZeusInterimPrel fromLeafNode(RelNode origin, PlanNode planNode) {
    return new ZeusInterimPrel(origin.getCluster(), origin.getTraitSet(), null, planNode);
  }
}

