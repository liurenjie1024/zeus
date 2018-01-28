package io.github.zeus;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.github.zeus.client.ZeusClient;
import io.github.zeus.client.ZeusClientBuilder;
import org.apache.calcite.schema.SchemaPlus;
import org.apache.drill.common.JSONOptions;
import org.apache.drill.common.logical.StoragePluginConfig;
import org.apache.drill.exec.server.DrillbitContext;
import org.apache.drill.exec.store.AbstractStoragePlugin;
import org.apache.drill.exec.store.SchemaConfig;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

/**
 * Created by liurenjie on 24/01/2018.
 */
public class ZeusStoragePlugin extends AbstractStoragePlugin {
  private static final Logger logger = LoggerFactory.getLogger(ZeusStoragePlugin.class);

  private final ZeusStoragePluginConfig config;
  private final DrillbitContext context;
  private final String name;
  private ZeusClient client;

  public ZeusStoragePlugin(ZeusStoragePluginConfig config, DrillbitContext context, String name) {
    this.config = config;
    this.context = context;
    this.name = name;
  }


  @Override
  public void registerSchemas(SchemaConfig schemaConfig, SchemaPlus parent) throws IOException {
    ZeusDB schema = new ZeusDB(this, name, client.getDBSchema(name));

    parent.add(name, schema);
  }

  @Override
  public StoragePluginConfig getConfig() {
    return config;
  }

  @Override
  public void start() {
    client = ZeusClientBuilder.newBuilder(config.getMetaHostname(),
      config.getMeataPort(),
      config.getDataHostname(),
      config.getDataPort())
      .build();
  }

  public ZeusClient getClient() {
    return client;
  }

  @Override
  public boolean supportsRead() {
    return true;
  }

  @Override
  public boolean supportsWrite() {
    return false;
  }

  @Override
  public ZeusGroupScan getPhysicalScan(String userName, JSONOptions selection) throws IOException {
    ZeusGroupScanSpec scanSpec = selection.getListWith(new ObjectMapper(),
      new TypeReference<ZeusGroupScanSpec>() {});

    return new ZeusGroupScan(name, scanSpec.getTableName(), null, config, this);
  }
  @Override
  public void close() {
    if (client != null) {
      try {
        client.close();
      } catch (Exception e) {
        logger.error("Failed to close zeus client.", e);
      }
    }
  }
}

