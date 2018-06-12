package io.github.zeus.rule;

import io.github.zeus.rpc.PlanNode;
import org.apache.calcite.plan.RelOptCluster;
import org.apache.calcite.plan.RelTraitSet;
import org.apache.calcite.rel.AbstractRelNode;
import org.apache.calcite.rel.RelNode;

public class ZeusScanPrel extends AbstractRelNode {
  private final PlanNode scanNode;

  /**
   * Creates an <code>AbstractRelNode</code>.
   *  @param cluster
   * @param traitSet
   * @param scanNode
   */
  public ZeusScanPrel(RelOptCluster cluster, RelTraitSet traitSet, PlanNode scanNode) {
    super(cluster, traitSet);
    this.scanNode = scanNode;
  }

  public static ZeusScanPrel from(RelNode origin, PlanNode planNode) {
    return new ZeusScanPrel(origin.getCluster(), origin.getTraitSet(), planNode);
  }
}
