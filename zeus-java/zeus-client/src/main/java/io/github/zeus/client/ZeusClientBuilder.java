package io.github.zeus.client;

import io.github.zeus.rpc.ZeusDataServiceGrpc;
import io.github.zeus.rpc.ZeusMetaServiceGrpc;
import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;

/**
 * Created by liurenjie on 24/01/2018.
 */
public class ZeusClientBuilder {
  private final String metaHost;
  private final int metaPort;
  private final String dataHost;
  private final int dataPort;


  private ZeusClientBuilder(String metaHost, int metaPort, String dataHost, int dataPort) {
    this.metaHost = metaHost;
    this.metaPort = metaPort;
    this.dataHost = dataHost;
    this.dataPort = dataPort;
  }

  public static ZeusClientBuilder newBuilder(String metaHost, int metaPort, String dataHost, int dataPort) {
    return new ZeusClientBuilder(metaHost, metaPort, dataHost, dataPort);
  }

  public ZeusClient build() {
    ManagedChannel metaChannel = ManagedChannelBuilder.forAddress(metaHost, metaPort)
      .usePlaintext(false)
      .build();
    ManagedChannel dataChannel = ManagedChannelBuilder.forAddress(dataHost, dataPort)
      .usePlaintext(false)
      .build();
    return new ZeusClient(metaChannel, ZeusMetaServiceGrpc.newBlockingStub(metaChannel),
      dataChannel, ZeusDataServiceGrpc.newBlockingStub(dataChannel));
  }
}
