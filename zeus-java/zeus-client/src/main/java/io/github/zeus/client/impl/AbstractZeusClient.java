package io.github.zeus.client.impl;

import io.github.zeus.client.ZeusClient;
import io.github.zeus.client.exception.CatalogNotFoundException;
import io.github.zeus.client.meta.ColumnMeta;
import io.github.zeus.client.meta.ResultMetadata;
import io.github.zeus.rpc.AggregationNode;
import io.github.zeus.rpc.ColumnType;
import io.github.zeus.rpc.GetRowNumNode;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.ProjectNode;
import io.github.zeus.rpc.QueryPlan;
import io.github.zeus.rpc.ScanNode;
import io.github.zeus.rpc.ZeusTableSchema;

import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

public abstract class AbstractZeusClient implements ZeusClient {
  @Override
  public ResultMetadata getResultMeta(QueryPlan plan) {
    return getResultMeta(plan.getRoot());
  }

  private ResultMetadata getResultMeta(PlanNode planNode) {
    switch (planNode.getPlanNodeType()) {
      case SCAN_NODE:
        return resultMetaOf(planNode.getScanNode());
      case PROJECT_NODE:
        return resultMetaOf(planNode.getProjectNode());
      case AGGREGATE_NODE:
        return resultMetaOf(planNode.getAggNode());
      case FILTER_NODE:
      case TOPN_NODE:
      case LIMIT_NODE:
        return getResultMeta(planNode.getChildren(0));
      case GET_ROW_NUM_NODE:
        return resultMetaOf(planNode.getGetRowNumNode());
      default:
        throw new IllegalArgumentException("Unrecognized node type: " + planNode.getPlanNodeType());
    }
  }

  private ResultMetadata resultMetaOf(ScanNode scanNode) {
    ZeusTableSchema tableSchema = getCatalog().getDbSchemasList().stream()
      .filter(db -> db.getId() == scanNode.getDbId())
      .findFirst()
      .map(db ->  db.getTablesOrThrow(scanNode.getTableId()))
      .orElseThrow(
        () -> CatalogNotFoundException.tableIdNotFound(scanNode.getDbId(), scanNode.getTableId()));

    List<ColumnMeta> columnMetas = scanNode.getColumnsList().stream()
      .map(tableSchema::getColumnsOrThrow)
      .map(column -> new ColumnMeta(column.getName(), column.getColumnType()))
      .collect(Collectors.toList());

    return new ResultMetadata(columnMetas);
  }

  private ResultMetadata resultMetaOf(ProjectNode projectNode) {
    List<ColumnMeta> columnMetas = projectNode.getItemsList().stream()
      .map(ColumnMeta::from)
      .collect(Collectors.toList());

    return new ResultMetadata(columnMetas);
  }

  private ResultMetadata resultMetaOf(AggregationNode node) {
    List<ColumnMeta> columnMetas = Arrays.asList(node.getGroupByList(), node.getAggFuncList())
      .stream()
      .flatMap(List::stream)
      .map(ColumnMeta::from)
      .collect(Collectors.toList());

    return new ResultMetadata(columnMetas);
  }

  private ResultMetadata resultMetaOf(GetRowNumNode node) {
    ColumnMeta columnMeta = new ColumnMeta(node.getAlias(), ColumnType.INT64);

    return new ResultMetadata(Collections.singletonList(columnMeta));
  }
}
