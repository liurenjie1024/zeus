package io.github.zeus.client;

import io.github.zeus.client.meta.ResultMetadata;
import io.github.zeus.rpc.GetRowNumNode;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.PlanNodeType;
import io.github.zeus.rpc.QueryPlan;
import io.github.zeus.rpc.QueryResult;
import io.github.zeus.rpc.ZeusCatalog;
import io.github.zeus.rpc.ZeusDBSchema;

import java.util.Optional;
import java.util.UUID;

/**
 * Created by liurenjie on 06/04/2018.
 */
public interface ZeusClient extends AutoCloseable {
  ZeusCatalog getCatalog();
  QueryResult query(QueryPlan plan);
  ResultMetadata getResultMeta(QueryPlan plan);

  default Optional<ZeusDBSchema> getDBSchema(String dbName) {
    return getCatalog().getDbSchemasList().stream()
      .filter(db -> db.getName().equals(dbName))
      .findFirst();
  }

  default long estimateRowCount(int dbId, int tableId) {
    final String name = "row_count";

    GetRowNumNode node = GetRowNumNode.newBuilder()
      .setAlias("row_count")
      .setDbId(dbId)
      .setTableId(tableId)
      .build();

    PlanNode planNode = PlanNode.newBuilder()
      .setPlanNodeType(PlanNodeType.GET_ROW_NUM_NODE)
      .setGetRowNumNode(node)
      .setNodeId(0)
      .build();

    QueryPlan plan = QueryPlan.newBuilder()
      .setPlanId(UUID.randomUUID().toString())
      .setRoot(planNode)
      .build();

    return query(plan).getRows(0)
      .getColumns(0)
      .getI64Value();
  }
}
