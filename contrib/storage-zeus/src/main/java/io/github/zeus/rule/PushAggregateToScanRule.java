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
import com.google.common.collect.Lists;
import io.github.zeus.ZeusGroupScan;
import io.github.zeus.expr.ZeusExprBuilder;
import io.github.zeus.rpc.AggregationNode;
import io.github.zeus.rpc.Expression;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.PlanNodeType;
import io.github.zeus.schema.ZeusTable;
import org.apache.calcite.plan.RelOptRuleCall;
import org.apache.calcite.plan.RelTraitSet;
import org.apache.calcite.rel.InvalidRelException;
import org.apache.calcite.rel.RelNode;
import org.apache.calcite.util.ImmutableBitSet;
import org.apache.drill.common.logical.data.NamedExpression;
import org.apache.drill.exec.physical.base.GroupScan;
import org.apache.drill.exec.planner.logical.DrillAggregateRel;
import org.apache.drill.exec.planner.logical.DrillScanRel;
import org.apache.drill.exec.planner.logical.RelOptHelper;
import org.apache.drill.exec.planner.physical.AggPrelBase.OperatorPhase;
import org.apache.drill.exec.planner.physical.AggPruleBase;
import org.apache.drill.exec.planner.physical.DrillDistributionTrait;
import org.apache.drill.exec.planner.physical.DrillDistributionTraitDef;
import org.apache.drill.exec.planner.physical.DrillScanPrel;
import org.apache.drill.exec.planner.physical.HashAggPrel;
import org.apache.drill.exec.planner.physical.HashToRandomExchangePrel;
import org.apache.drill.exec.planner.physical.Prel;
import org.apache.drill.exec.planner.physical.PrelUtil;
import org.apache.drill.exec.planner.physical.ScanPrel;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

public class PushAggregateToScanRule extends AggPruleBase {
  public static final PushAggregateToScanRule SINGLETON = new PushAggregateToScanRule();

  private static final Logger LOG = LoggerFactory.getLogger(PushAggregateToScanRule.class);

  private PushAggregateToScanRule() {
    super(RelOptHelper.some(DrillAggregateRel.class, RelOptHelper.any(DrillScanRel.class)),
      "PushAggregateToScanRule");
  }

  @Override
  public void onMatch(RelOptRuleCall call) {
    if (!PrelUtil.getPlannerSettings(call.getPlanner()).isHashAggEnabled()) {
      return;
    }

    final DrillAggregateRel aggregate = call.rel(0);
    final DrillScanRel scan = call.rel(1);

    if (aggregate.containsDistinctCall() || aggregate.getGroupCount() == 0) {
      // currently, don't use HashAggregate if any of the logical aggrs contains DISTINCT or
      // if there are no grouping keys
      return;
    }

    DrillDistributionTrait distOnAllKeys =
        new DrillDistributionTrait(DrillDistributionTrait.DistributionType.HASH_DISTRIBUTED,
            ImmutableList.copyOf(getDistributionField(aggregate, true /* get all grouping keys */)));
    RelTraitSet traits = call.getPlanner().emptyTraitSet()
        .plus(Prel.DRILL_PHYSICAL)
        .plus(scan.getTraitSet().getTrait(DrillDistributionTraitDef.INSTANCE));

    try {
      HashAggPrel phase1Agg = new HashAggPrel(
          aggregate.getCluster(),
          traits,
          scan,
          aggregate.indicator,
          aggregate.getGroupSet(),
          aggregate.getGroupSets(),
          aggregate.getAggCallList(),
          OperatorPhase.PHASE_1of2);


      ScanPrel newScan = pushAggToScan(phase1Agg, scan);

      if (newScan == null) {
        return;
      }

      HashToRandomExchangePrel exch =
          new HashToRandomExchangePrel(newScan.getCluster(), newScan.getTraitSet().plus(Prel.DRILL_PHYSICAL).plus(distOnAllKeys),
              newScan, ImmutableList.copyOf(getDistributionField(aggregate, true)));

      ImmutableBitSet newGroupSet = remapGroupSet(aggregate.getGroupSet());
      List<ImmutableBitSet> newGroupSets = Lists.newArrayList();
      for (ImmutableBitSet groupSet : aggregate.getGroupSets()) {
        newGroupSets.add(remapGroupSet(groupSet));
      }

      RelNode r = new HashAggPrel(
          aggregate.getCluster(),
          exch.getTraitSet(),
          exch,
          aggregate.indicator,
          newGroupSet,
          newGroupSets,
          phase1Agg.getPhase2AggCalls(),
          OperatorPhase.PHASE_2of2);

      call.transformTo(r);
    } catch (InvalidRelException e) {
      LOG.error("Failed to push aggregate down to scan.", e);
    }
  }

  private static ScanPrel pushAggToScan(HashAggPrel hashAggPrel, DrillScanRel scanPrel) {
    ZeusGroupScan zeusGroupScan = (ZeusGroupScan) scanPrel.getGroupScan();
    ZeusTable table = zeusGroupScan.getTable();

    boolean allConverted = true;
    List<Expression> groupBys = new ArrayList<>(hashAggPrel.getKeys().size());
    for (NamedExpression key : hashAggPrel.getKeys()) {
      Optional<Expression> groupBy = key.getExpr().accept(new ZeusExprBuilder(table), null);
      if (!groupBy.isPresent()) {
        allConverted = false;
        break;
      }

      groupBys.add(groupBy.get());
    }

    if (!allConverted) {
      return null;
    }

    List<Expression> aggs = new ArrayList<>(hashAggPrel.getAggExprs().size());
    for (NamedExpression agg : hashAggPrel.getAggExprs()) {
      Optional<Expression> aggExpr = agg.getExpr().accept(new ZeusExprBuilder(table), null);
      if (!aggExpr.isPresent()) {
        allConverted = false;
        break;
      }

      Expression expr = Expression.newBuilder(aggExpr.get())
          .setAlias(agg.getRef().getLastSegment().getNameSegment().getPath())
          .build();
      aggs.add(expr);
    }

    if (!allConverted) {
      return null;
    }

    AggregationNode aggNode = AggregationNode.newBuilder()
        .addAllGroupBy(groupBys)
        .addAllAggFunc(aggs)
        .build();

    PlanNode newRoot = PlanNode.newBuilder()
        .setAggNode(aggNode)
        .setPlanNodeType(PlanNodeType.AGGREGATE_NODE)
        .build();

    ZeusGroupScan newGroupScan = zeusGroupScan.cloneWithNewRootPlanNode(newRoot);
    newGroupScan.setRowCount(5);
    newGroupScan.setAggPushedDown(true);

    ScanPrel newScan = ScanPrel.create(scanPrel,
        hashAggPrel.getTraitSet(), newGroupScan, hashAggPrel.getRowType());

    return newScan;
  }


  @Override
  public boolean matches(RelOptRuleCall call) {
     DrillScanRel scanRel = call.rel(1);

    GroupScan groupScan = scanRel.getGroupScan();
    if (!(groupScan instanceof ZeusGroupScan)) {
      return false;
    }

//    ZeusGroupScan zeusGroupScan = (ZeusGroupScan) groupScan;
//    if (zeusGroupScan.isAggPushedDown()) {
//      return false;
//    }

    return super.matches(call);
  }
}
