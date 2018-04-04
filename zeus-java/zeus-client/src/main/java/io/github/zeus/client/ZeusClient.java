package io.github.zeus.client;

import io.github.zeus.rpc.*;
import io.github.zeus.rpc.ZeusDataServiceGrpc.ZeusDataServiceBlockingStub;
import io.github.zeus.rpc.ZeusMetaServiceGrpc.ZeusMetaServiceBlockingStub;
import io.grpc.ManagedChannel;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

/**
 * Created by liurenjie on 24/01/2018.
 */
public class ZeusClient implements AutoCloseable {
  private static final Logger logger = LoggerFactory.getLogger(ZeusClient.class);

  private final ZeusCatalog catalog;
  private final ManagedChannel dataChannel;
  private final ZeusDataServiceBlockingStub dataRpcClient;

  public ZeusClient(ZeusCatalog catalog,
                    ManagedChannel dataChannel,
                    ZeusDataServiceBlockingStub dataRpcClient) {
    this.catalog = catalog;
    this.dataChannel = dataChannel;
    this.dataRpcClient = dataRpcClient;
  }


  public ZeusDBSchema getDBSchema(String db) {
    return catalog.getDbSchemasList()
        .stream()
        .filter(s -> s.getName().equals(db))
        .findFirst()
        .get();
  }

  public QueryResult query(QueryPlan plan) {
    return dataRpcClient.query(QueryRequest.newBuilder()
      .setPlan(plan)
      .build());
  }

  public void close() throws Exception {
    dataChannel.shutdownNow();
  }

  public static void main(String[] args) throws IOException {
    System.setProperty("org.slf4j.simpleLogger.logFile", "System.out");
    ZeusClient client = ZeusClientBuilder.newBuilder("/home/liurenjie-sal/Workspace/MyCodes/zeus/zeus-server/src/bin/data/test.schema", "127.0.0.1", 8899)
      .build();

    PlanNode node = PlanNode.newBuilder()
      .setPlanNodeType(PlanNodeType.SCAN_NODE)
      .setScanNode(ScanNode.newBuilder()
              .setDbId(1)
              .setTableId(1)
              .addColumns(1)
              .addColumns(2)
              .addColumns(3)
              .addColumns(4)
              .addColumns(5)
              .addColumns(6)
              .build())
      .build();

    QueryPlan plan = QueryPlan.newBuilder()
      .setPlanId(1).setRoot(node)
      .build();

    QueryResult result = client.query(plan);

    System.out.println(result.getCode());

    result.getRowsList().stream()
      .flatMap(row -> row.getColumnsList().stream())
      .forEach(System.out::println);
  }
}
