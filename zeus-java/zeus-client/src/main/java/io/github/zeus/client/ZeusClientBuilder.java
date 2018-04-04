package io.github.zeus.client;

import io.github.zeus.rpc.ZeusCatalog;
import io.github.zeus.rpc.ZeusDataServiceGrpc;
import io.github.zeus.rpc.ZeusMetaServiceGrpc;
import io.github.zeus.rpc.ZeusMetaServiceGrpc.ZeusMetaServiceBlockingStub;
import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;

/**
 * Created by liurenjie on 24/01/2018.
 */
public class ZeusClientBuilder {
  private final String schemaPath;
  private final String dataHost;
  private final int dataPort;


  private ZeusClientBuilder(String schemaPath, String dataHost, int dataPort) {
    this.schemaPath = schemaPath;
    this.dataHost = dataHost;
    this.dataPort = dataPort;
  }

  public static ZeusClientBuilder newBuilder(String schemaPath, String dataHost, int dataPort) {
    return new ZeusClientBuilder(schemaPath, dataHost, dataPort);
  }

  public ZeusClient build() throws IOException {
    try (FileInputStream f = new FileInputStream(this.schemaPath)) {
      ZeusCatalog catalog = ZeusCatalog.parseFrom(f);
      ManagedChannel dataChannel = ManagedChannelBuilder.forAddress(dataHost, dataPort)
          .usePlaintext(true)
          .build();
      return new ZeusClient(catalog,
          dataChannel,
          ZeusDataServiceGrpc.newBlockingStub(dataChannel));
    }
  }
}
