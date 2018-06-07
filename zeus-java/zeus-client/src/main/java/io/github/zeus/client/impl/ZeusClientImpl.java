package io.github.zeus.client.impl;

import io.github.zeus.client.ZeusClient;
import io.github.zeus.client.ZeusClientBuilder;
import io.github.zeus.client.exception.CatalogNotFoundException;
import io.github.zeus.client.meta.ColumnMeta;
import io.github.zeus.client.meta.ResultMetadata;
import io.github.zeus.rpc.*;
import io.github.zeus.rpc.ZeusDataServiceGrpc.ZeusDataServiceBlockingStub;
import io.grpc.ManagedChannel;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import static io.github.zeus.rpc.PlanNodeType.AGGREGATE_NODE;
import static io.github.zeus.rpc.PlanNodeType.FILTER_NODE;
import static io.github.zeus.rpc.PlanNodeType.LIMIT_NODE;
import static io.github.zeus.rpc.PlanNodeType.PROJECT_NODE;
import static io.github.zeus.rpc.PlanNodeType.SCAN_NODE;
import static io.github.zeus.rpc.PlanNodeType.TOPN_NODE;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import java.util.UUID;
import java.util.stream.Collectors;

/**
 * Created by liurenjie on 24/01/2018.
 */
public class ZeusClientImpl implements ZeusClient {
  private static final Logger logger = LoggerFactory.getLogger(ZeusClientImpl.class);

  private final ZeusCatalog catalog;
  private final ManagedChannel dataChannel;
  private final ZeusDataServiceBlockingStub dataRpcClient;

  public ZeusClientImpl(ZeusCatalog catalog,
                        ManagedChannel dataChannel,
                        ZeusDataServiceBlockingStub dataRpcClient) {
    this.catalog = catalog;
    this.dataChannel = dataChannel;
    this.dataRpcClient = dataRpcClient;
  }


  public Optional<ZeusDBSchema > getDBSchema(String db) {
    return catalog.getDbSchemasList()
        .stream()
        .filter(s -> s.getName().equals(db))
        .findFirst();
  }

  public QueryResult query(QueryPlan plan) {
    return dataRpcClient.query(QueryRequest.newBuilder()
      .setPlan(plan)
      .build());
  }

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
      default:
          throw new IllegalArgumentException("Unrecognized node type: " + planNode.getPlanNodeType());
    }
  }

  private ResultMetadata resultMetaOf(ScanNode scanNode) {
    ZeusTableSchema tableSchema = catalog.getDbSchemasList().stream()
        .filter(db -> db.getId() == scanNode.getDbId())
        .findFirst()
        .map(db ->  db.getTablesOrThrow(scanNode.getTableId()))
        .orElseThrow(
            () -> CatalogNotFoundException.tableIdNotFound(scanNode.getDbId(), scanNode.getTableId()));

    List<ColumnMeta> columnMetas = scanNode.getColumnsList().stream()
        .map(columnId -> tableSchema.getColumnsOrThrow(columnId))
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


  public void close() throws Exception {
    dataChannel.shutdownNow();
  }

  public static void main(String[] args) throws IOException {
    System.setProperty("org.slf4j.simpleLogger.logFile", "System.out");
    ZeusClient client = ZeusClientBuilder.newBuilder("/home/liurenjie-sal/Downloads/test/logs.schema", "127.0.0.1", 8899)
      .build();

    PlanNode node = PlanNode.newBuilder()
      .setPlanNodeType(PlanNodeType.SCAN_NODE)
      .setScanNode(ScanNode.newBuilder()
              .setDbId(1)
              .setTableId(1)
              .addColumns(14)
              .build())
      .build();

    QueryPlan plan = QueryPlan.newBuilder()
      .setPlanId(UUID.randomUUID().toString()).setRoot(node)
      .build();

    QueryResult result = client.query(plan);

    System.out.println(result.getCode());

    result.getRowsList().stream()
      .flatMap(row -> row.getColumnsList().stream())
      .forEach(System.out::println);
  }
}
