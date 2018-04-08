/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package io.github.zeus;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonTypeName;
import com.google.common.annotations.VisibleForTesting;
import io.github.zeus.client.ZeusClient;
import io.github.zeus.client.ZeusClientBuilder;
import org.apache.drill.common.logical.StoragePluginConfigBase;

import java.io.IOException;
import java.util.Objects;

@JsonTypeName(ZeusStoragePluginConfig.NAME)
public class ZeusStoragePluginConfig extends StoragePluginConfigBase {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(ZeusStoragePluginConfig.class);

  public static final String NAME = "zeus";

  private final String schemaPath;
  private final String dataHostname;
  private final int dataPort;

  private ZeusClient client; // Only for testing, in the future we will disable set client directly

  @JsonCreator
  public ZeusStoragePluginConfig(@JsonProperty("schemaPath") String schemaPath,
                                 @JsonProperty("dataHostname") String dataHostname,
                                 @JsonProperty("dataPort") int dataPort) {
    this.schemaPath = schemaPath;
    this.dataHostname = dataHostname;
    this.dataPort = dataPort;
  }


  @JsonProperty
  public String getSchemaPath() {
    return schemaPath;
  }

  @JsonProperty
  public String getDataHostname() {
    return dataHostname;
  }

  @JsonProperty
  public int getDataPort() {
    return dataPort;
  }

  @JsonIgnore
  public ZeusClient getClient() throws IOException {
    if (client != null) {
      return client;
    } else {
      client = ZeusClientBuilder.newBuilder(getSchemaPath(),
        getDataHostname(),
        getDataPort())
        .build();
      return client;
    }
  }

  @VisibleForTesting
  void setClient(ZeusClient client) {
    this.client = client;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) return true;
    if (o == null || getClass() != o.getClass()) return false;
    ZeusStoragePluginConfig that = (ZeusStoragePluginConfig) o;
    return getDataPort() == that.getDataPort() &&
      Objects.equals(getDataHostname(), that.getDataHostname()) &&
        Objects.equals(getSchemaPath(), that.getSchemaPath());
  }

  @Override
  public int hashCode() {
    return Objects.hash(getSchemaPath(),
      getDataHostname(), getDataPort());
  }

  @Override
  public String toString() {
    return "ZeusStoragePluginConfig{" +
        "schemaPath='" + schemaPath + '\'' +
        ", dataHostname='" + dataHostname + '\'' +
        ", dataPort=" + dataPort +
        '}';
  }
}
