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
import io.github.zeus.ZeusGroupScan;
import io.github.zeus.expr.ZeusExprBuilder;
import io.github.zeus.rpc.AggregationNode;
import io.github.zeus.rpc.Expression;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.PlanNodeType;
import io.github.zeus.schema.ZeusTable;
import org.apache.calcite.plan.RelOptRule;
import org.apache.calcite.plan.RelOptRuleCall;
import org.apache.calcite.rel.RelNode;
import org.apache.drill.common.logical.data.NamedExpression;
import org.apache.drill.exec.physical.base.GroupScan;
import org.apache.drill.exec.planner.physical.HashAggPrel;
import org.apache.drill.exec.planner.physical.HashToRandomExchangePrel;
import org.apache.drill.exec.planner.physical.ScanPrel;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

public class PushAggregateToScanRule extends RelOptRule {
  public static final PushAggregateToScanRule  SINGLETON = new PushAggregateToScanRule();

  private PushAggregateToScanRule() {
    super(RelOptRule.operand(HashAggPrel.class, RelOptRule.operand(HashToRandomExchangePrel.class,
        RelOptRule.operand(ScanPrel.class, RelOptRule.none()))));
  }

  @Override
  public void onMatch(RelOptRuleCall call) {
    HashAggPrel hashAggPrel = call.rel(0);
    HashToRandomExchangePrel hashPrel = call.rel(1);
    ScanPrel scanPrel = call.rel(2);

    ZeusGroupScan zeusGroupScan = (ZeusGroupScan) scanPrel.getGroupScan();
    ZeusTable table = zeusGroupScan.getTable();

    boolean allConverted = true;
    List<Expression> groupBys = new ArrayList<>(hashAggPrel.getKeys().size());
    for (NamedExpression key: hashAggPrel.getKeys()) {
      Optional<Expression> groupBy = key.getExpr().accept(new ZeusExprBuilder(table), null);
      if (!groupBy.isPresent()) {
        allConverted = false;
        break;
      }

      groupBys.add(groupBy.get());
    }

    if (!allConverted) {
      return;
    }

    List<Expression> aggs = new ArrayList<>(hashAggPrel.getAggExprs().size());
    for (NamedExpression agg: hashAggPrel.getAggExprs()) {
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
      return;
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

    ScanPrel newScan = ScanPrel.create(scanPrel,
        hashAggPrel.getTraitSet(), newGroupScan, hashAggPrel.getRowType());
    RelNode newHash = hashPrel.copy(hashPrel.getTraitSet(), ImmutableList.of(newScan));
    RelNode newHashAgg = newHash.copy(hashAggPrel.getTraitSet(), ImmutableList.of(newHash));

    call.transformTo(newHashAgg);
  }

  @Override
  public boolean matches(RelOptRuleCall call) {
    ScanPrel scanPrel = call.rel(2);

    GroupScan groupScan = scanPrel.getGroupScan();
    if (!(groupScan instanceof ZeusGroupScan)) {
      return false;
    }

    return super.matches(call);
  }
}
