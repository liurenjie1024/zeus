package io.github.zeus.client.impl;

import io.github.zeus.client.ZeusClient;
import io.github.zeus.client.ZeusClientBuilder;
import io.github.zeus.rpc.*;
import io.github.zeus.rpc.ZeusDataServiceGrpc.ZeusDataServiceBlockingStub;
import io.grpc.ManagedChannel;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Optional;
import java.util.UUID;

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

  public void close() throws Exception {
    dataChannel.shutdownNow();
  }

  public static void main(String[] args) throws IOException {
    System.setProperty("org.slf4j.simpleLogger.logFile", "System.out");
    ZeusClient client = ZeusClientBuilder.newBuilder("/home/liurenjie-sal/Downloads/test/test.schema", "127.0.0.1", 8899)
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
              .addColumns(4)
              .addColumns(6)
              .addColumns(7)
              .addColumns(8)
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
