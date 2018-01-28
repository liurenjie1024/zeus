package io.github.zeus;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;

/**
 * Created by liurenjie on 28/01/2018.
 */
public class ZeusGroupScanSpec {
  private final String tableName;

  @JsonCreator
  public ZeusGroupScanSpec(@JsonProperty("tableName") String tableName) {
    this.tableName = tableName;
  }

  @JsonProperty
  String getTableName() {
    return tableName;
  }
}
