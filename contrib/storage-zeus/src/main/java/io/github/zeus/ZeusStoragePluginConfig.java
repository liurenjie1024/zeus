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
