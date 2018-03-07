package io.github.zeus.client;

import io.github.zeus.rpc.ColumnValue;
import io.github.zeus.rpc.GetSchemaRequest;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.PlanNodeType;
import io.github.zeus.rpc.QueryPlan;
import io.github.zeus.rpc.QueryRequest;
import io.github.zeus.rpc.QueryResult;
import io.github.zeus.rpc.ScanNode;
import io.github.zeus.rpc.ZeusDBSchema;
import io.github.zeus.rpc.ZeusDataServiceGrpc.ZeusDataServiceBlockingStub;
import io.github.zeus.rpc.ZeusMetaServiceGrpc.ZeusMetaServiceBlockingStub;
import io.grpc.ManagedChannel;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * Created by liurenjie on 24/01/2018.
 */
public class ZeusClient implements AutoCloseable {
  private static final Logger logger = LoggerFactory.getLogger(ZeusClient.class);

  private final ManagedChannel metaChannel;
  private final ZeusMetaServiceBlockingStub metaRpcClient;
  private final ManagedChannel dataChannel;
  private final ZeusDataServiceBlockingStub dataRpcClient;

  public ZeusClient(ManagedChannel metaChannel,
                    ZeusMetaServiceBlockingStub metaRpcClient,
                    ManagedChannel dataChannel,
                    ZeusDataServiceBlockingStub dataRpcClient) {
    this.metaChannel = metaChannel;
    this.metaRpcClient = metaRpcClient;
    this.dataChannel = dataChannel;
    this.dataRpcClient = dataRpcClient;
  }


  public ZeusDBSchema getDBSchema(String db) {
    GetSchemaRequest request = GetSchemaRequest.newBuilder()
      .setDbName(db)
      .build();
    return metaRpcClient.getDBSchema(request)
      .getDbSchema();
  }

  public QueryResult query(QueryPlan plan) {
    return dataRpcClient.query(QueryRequest.newBuilder()
      .setPlan(plan)
      .build());
  }

  public void close() throws Exception {
    metaChannel.shutdownNow();
    dataChannel.shutdownNow();
  }

  public static void main(String[] args) {
    System.setProperty("org.slf4j.simpleLogger.logFile", "System.out");
    ZeusClient client = ZeusClientBuilder.newBuilder(null, 0, "127.0.0.1", 7788)
      .build();

    PlanNode node = PlanNode.newBuilder()
      .setPlanNodeType(PlanNodeType.SCAN_NODE)
      .setScanNode(ScanNode.newBuilder().addColumns(1).build())
      .build();

    QueryPlan plan = QueryPlan.newBuilder()
      .setPlanId(1)
      .addNodes(node)
      .build();

    QueryResult result = client.query(plan);
    result.getRowsList().stream()
      .flatMap(row -> row.getColumnsList().stream())
      .map(ColumnValue::getStringValue)
      .forEach(System.out::println);
  }
}
