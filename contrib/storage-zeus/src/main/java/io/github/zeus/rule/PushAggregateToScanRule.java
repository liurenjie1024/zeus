/**
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
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
import org.apache.calcite.plan.RelOptRuleCall;
import org.apache.calcite.plan.RelTrait;
import org.apache.calcite.plan.RelTraitSet;
import org.apache.calcite.rel.InvalidRelException;
import org.apache.calcite.rel.RelNode;
import org.apache.calcite.util.ImmutableBitSet;
import org.apache.drill.exec.physical.base.GroupScan;
import org.apache.drill.exec.planner.logical.DrillAggregateRel;
import org.apache.drill.exec.planner.logical.DrillScanRel;
import org.apache.drill.exec.planner.logical.RelOptHelper;
import org.apache.drill.exec.planner.physical.AggPrelBase.OperatorPhase;
import org.apache.drill.exec.planner.physical.AggPruleBase;
import org.apache.drill.exec.planner.physical.DrillDistributionTrait;
import org.apache.drill.exec.planner.physical.DrillDistributionTraitDef;
import org.apache.drill.exec.planner.physical.HashAggPrel;
import org.apache.drill.exec.planner.physical.HashToRandomExchangePrel;
import org.apache.drill.exec.planner.physical.Prel;
import org.apache.drill.exec.planner.physical.PrelUtil;
import org.apache.drill.exec.planner.physical.ScanPrel;
import org.apache.drill.exec.planner.physical.SubsetTransformer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;

public class PushAggregateToScanRule extends AggPruleBase {
  public static final PushAggregateToScanRule SINGLETON = new PushAggregateToScanRule();

  private static final Logger LOG = LoggerFactory.getLogger(PushAggregateToScanRule.class);

  private PushAggregateToScanRule() {
    super(RelOptHelper.some(DrillAggregateRel.class, RelOptHelper.any(DrillScanRel.class)), "PushAggregateToScanRule");
  }

  @Override
  public void onMatch(RelOptRuleCall call) {
    if (!PrelUtil.getPlannerSettings(call.getPlanner()).isHashAggEnabled()) {
      return;
    }

    final DrillAggregateRel aggregate = call.rel(0);
    final RelNode input = call.rel(1);

    if (aggregate.containsDistinctCall() || aggregate.getGroupCount() == 0) {
      // currently, don't use HashAggregate if any of the logical aggrs contains DISTINCT or
      // if there are no grouping keys
      return;
    }

    RelTraitSet traits;

    try {
      if (aggregate.getGroupSet().isEmpty()) {
        DrillDistributionTrait singleDist = DrillDistributionTrait.SINGLETON;
        traits = call.getPlanner().emptyTraitSet().plus(Prel.DRILL_PHYSICAL).plus(singleDist);
        createTransformRequest(call, aggregate, input, traits);
      } else {
        // hash distribute on all grouping keys
        DrillDistributionTrait distOnAllKeys =
            new DrillDistributionTrait(DrillDistributionTrait.DistributionType.HASH_DISTRIBUTED,
                ImmutableList.copyOf(getDistributionField(aggregate, true /* get all grouping keys */)));

        traits = call.getPlanner().emptyTraitSet().plus(Prel.DRILL_PHYSICAL);

        RelNode convertedInput = convert(input, traits);
        new TwoPhaseSubset(call, distOnAllKeys).go(aggregate, convertedInput);
      }
    } catch (InvalidRelException e) {
      LOG.warn("Failed to push aggregate to scan.", e);
    }
  }

  @Override
  public boolean matches(RelOptRuleCall call) {
    DrillScanRel scanRel = call.rel(1);

    GroupScan groupScan = scanRel.getGroupScan();
    if (!(groupScan instanceof ZeusGroupScan)) {
      return false;
    }

    return super.matches(call);
  }

  private void createTransformRequest(RelOptRuleCall call, DrillAggregateRel aggregate,
                                      RelNode input, RelTraitSet traits) throws InvalidRelException {

    final RelNode convertedInput = convert(input, PrelUtil.fixTraits(call, traits));

    HashAggPrel newAgg = new HashAggPrel(
        aggregate.getCluster(),
        traits,
        convertedInput,
        aggregate.indicator,
        aggregate.getGroupSet(),
        aggregate.getGroupSets(),
        aggregate.getAggCallList(),
        OperatorPhase.PHASE_1of1);

    call.transformTo(newAgg);
  }

  private class TwoPhaseSubset extends SubsetTransformer<DrillAggregateRel, InvalidRelException> {
    final RelTrait distOnAllKeys;

    public TwoPhaseSubset(RelOptRuleCall call, RelTrait distOnAllKeys) {
      super(call);
      this.distOnAllKeys = distOnAllKeys;
    }

    @Override
    public RelNode convertChild(DrillAggregateRel aggregate, RelNode input) throws InvalidRelException {

      RelTraitSet traits = newTraitSet(Prel.DRILL_PHYSICAL, input.getTraitSet().getTrait(DrillDistributionTraitDef.INSTANCE));
      ScanPrel newInput = (ScanPrel) convert(input, traits);

      HashAggPrel phase1Agg = new HashAggPrel(
          aggregate.getCluster(),
          traits,
          newInput,
          aggregate.indicator,
          aggregate.getGroupSet(),
          aggregate.getGroupSets(),
          aggregate.getAggCallList(),
          OperatorPhase.PHASE_1of2);

      HashToRandomExchangePrel exch =
          new HashToRandomExchangePrel(phase1Agg.getCluster(), phase1Agg.getTraitSet().plus(Prel.DRILL_PHYSICAL).plus(distOnAllKeys),
              phase1Agg, ImmutableList.copyOf(getDistributionField(aggregate, true)));

      ImmutableBitSet newGroupSet = remapGroupSet(aggregate.getGroupSet());
      List<ImmutableBitSet> newGroupSets = Lists.newArrayList();
      for (ImmutableBitSet groupSet : aggregate.getGroupSets()) {
        newGroupSets.add(remapGroupSet(groupSet));
      }

      return new HashAggPrel(
          aggregate.getCluster(),
          exch.getTraitSet(),
          exch,
          aggregate.indicator,
          newGroupSet,
          newGroupSets,
          phase1Agg.getPhase2AggCalls(),
          OperatorPhase.PHASE_2of2);
    }
  }
}
