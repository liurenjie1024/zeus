package io.github.zeus.client;

import io.github.zeus.rpc.GetSchemaRequest;
import io.github.zeus.rpc.QueryPlan;
import io.github.zeus.rpc.QueryRequest;
import io.github.zeus.rpc.QueryResult;
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
}
