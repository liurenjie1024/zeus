package io.github.zeus.rule;

import io.github.zeus.rpc.QueryPlan;
import org.apache.calcite.plan.RelOptCluster;
import org.apache.calcite.plan.RelTraitSet;
import org.apache.calcite.rel.AbstractRelNode;
import org.apache.drill.exec.physical.base.PhysicalOperator;
import org.apache.drill.exec.planner.physical.PhysicalPlanCreator;
import org.apache.drill.exec.planner.physical.Prel;
import org.apache.drill.exec.planner.physical.visitor.PrelVisitor;
import org.apache.drill.exec.record.BatchSchema.SelectionVectorMode;

import java.io.IOException;
import java.util.Collections;
import java.util.Iterator;

public class ZeusPrel extends AbstractRelNode implements Prel {

  private final QueryPlan queryPlan;

  /**
   * Creates an <code>AbstractRelNode</code>.
   *
   * @param cluster
   * @param traitSet
   */
  public ZeusPrel(RelOptCluster cluster, RelTraitSet traitSet, QueryPlan queryPlan) {
    super(cluster, traitSet);
    this.queryPlan = queryPlan;
  }


  @Override
  public PhysicalOperator getPhysicalOperator(PhysicalPlanCreator creator) throws IOException {
    return null;
  }

  @Override
  public <T, X, E extends Throwable> T accept(PrelVisitor<T, X, E> logicalVisitor, X value) throws E {
    return null;
  }

  @Override
  public SelectionVectorMode[] getSupportedEncodings() {
    return new SelectionVectorMode[0];
  }

  @Override
  public SelectionVectorMode getEncoding() {
    return null;
  }

  @Override
  public boolean needsFinalColumnReordering() {
    return false;
  }

  @Override
  public Iterator<Prel> iterator() {
    return Collections.emptyIterator();
  }
}
