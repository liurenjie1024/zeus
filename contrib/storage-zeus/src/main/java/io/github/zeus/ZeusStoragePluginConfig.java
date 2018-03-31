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
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonTypeName;
import org.apache.drill.common.logical.StoragePluginConfigBase;

import java.util.Objects;

@JsonTypeName(ZeusStoragePluginConfig.NAME)
public class ZeusStoragePluginConfig extends StoragePluginConfigBase {
  static final org.slf4j.Logger logger = org.slf4j.LoggerFactory.getLogger(ZeusStoragePluginConfig.class);

  public static final String NAME = "zeus";

  private final String metaHostname;
  private final int meataPort;
  private final String dataHostname;
  private final int dataPort;

  @JsonCreator
  public ZeusStoragePluginConfig(@JsonProperty("metaHostname") String metaHostname,
                                 @JsonProperty("meataPort") int meataPort,
                                 @JsonProperty("dataHostname") String dataHostname,
                                 @JsonProperty("dataPort") int dataPort) {
    this.metaHostname = metaHostname;
    this.meataPort = meataPort;
    this.dataHostname = dataHostname;
    this.dataPort = dataPort;
  }

  @JsonProperty
  public String getMetaHostname() {
    return metaHostname;
  }

  @JsonProperty
  public int getMeataPort() {
    return meataPort;
  }

  @JsonProperty
  public String getDataHostname() {
    return dataHostname;
  }

  @JsonProperty
  public int getDataPort() {
    return dataPort;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) return true;
    if (o == null || getClass() != o.getClass()) return false;
    ZeusStoragePluginConfig that = (ZeusStoragePluginConfig) o;
    return getMeataPort() == that.getMeataPort() &&
      getDataPort() == that.getDataPort() &&
      Objects.equals(getMetaHostname(), that.getMetaHostname()) &&
      Objects.equals(getDataHostname(), that.getDataHostname());
  }

  @Override
  public int hashCode() {
    return Objects.hash(getMetaHostname(), getMeataPort(),
      getDataHostname(), getDataPort());
  }
}
