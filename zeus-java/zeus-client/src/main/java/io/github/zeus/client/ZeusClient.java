package io.github.zeus.client;

import io.github.zeus.client.meta.ResultMetadata;
import io.github.zeus.rpc.QueryPlan;
import io.github.zeus.rpc.QueryResult;
import io.github.zeus.rpc.ZeusDBSchema;

import java.util.Optional;

/**
 * Created by liurenjie on 06/04/2018.
 */
public interface ZeusClient extends AutoCloseable {
  Optional<ZeusDBSchema> getDBSchema(String dbName);
  QueryResult query(QueryPlan plan);
  ResultMetadata getResultMeta(QueryPlan plan);
}
