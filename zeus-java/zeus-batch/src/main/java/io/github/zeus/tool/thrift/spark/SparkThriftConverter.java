package io.github.zeus.tool.thrift.spark;

import io.github.zeus.tool.thrift.DataTypeMappings;
import io.github.zeus.tool.thrift.SparkDataTypeMapping;
import io.github.zeus.tool.thrift.ThriftConverter;
import io.github.zeus.tool.thrift.ThriftDataTypeMapping;
import org.apache.spark.sql.Row;
import org.apache.spark.sql.RowFactory;
import org.apache.spark.sql.types.Metadata;
import org.apache.spark.sql.types.StructField;
import org.apache.spark.sql.types.StructType;
import org.apache.thrift.TBase;
import org.apache.thrift.TFieldIdEnum;
import org.apache.thrift.meta_data.FieldMetaData;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.Serializable;
import java.util.EnumSet;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

public class SparkThriftConverter<T extends TBase<T, F>, F extends Enum<F> & TFieldIdEnum>
  implements Serializable {
  private static final Logger LOGGER = LoggerFactory.getLogger(ThriftConverter.class);
  private final Class<T> klass;

  private EnumSet<F> fields;
  private Map<F, FieldMetaData> filedMetaDataMap;
  private StructType sparkSchema;

  public SparkThriftConverter(Class<T> klass) {
    this.klass = klass;
  }

  @SuppressWarnings(value = "unchecked")
  private void initSchema() {
    if (sparkSchema != null) {
      return;
    }

    try {
      Class<F> fieldEnumClass = (Class<F>) Class.forName(klass.getName() + "$_Fields");
      filedMetaDataMap = (Map<F, FieldMetaData>) klass.getField("metaDataMap")
        .get(null);
      fields = EnumSet.allOf(fieldEnumClass);


      StructField[] structFields = fields
        .stream()
        .map(f -> fieldSchemaOf(filedMetaDataMap.get(f)))
        .filter(Optional::isPresent)
        .map(Optional::get)
        .toArray(StructField[]::new);

      sparkSchema = new StructType(structFields);
    } catch (Exception e) {
      throw new IllegalArgumentException(e);
    }
  }

  public StructType getSchema() {
    initSchema();
    return sparkSchema;
  }

  private static Optional<StructField> fieldSchemaOf(FieldMetaData fieldMetaData) {
    return DataTypeMappings.thriftDataTypeMappingOf(fieldMetaData.valueMetaData.type)
      .flatMap(m -> DataTypeMappings.sparkDataTypeMappingOf(m.zeusType()))
      .map(m -> new StructField(fieldMetaData.fieldName, m.sparkType(),
        false,
        Metadata.empty()));
  }

  private Optional<SparkDataTypeMapping> sparkDataTypeMappingOf(F field) {
    initSchema();
    return DataTypeMappings.thriftDataTypeMappingOf(
      this.filedMetaDataMap.get(field).valueMetaData.type)
      .flatMap(m -> DataTypeMappings.sparkDataTypeMappingOf(m.zeusType()));
  }

  public Row createRow(T log) {
    return RowFactory.create(fields.stream()
      .map(f -> {
        Optional<SparkDataTypeMapping> typeMapping = sparkDataTypeMappingOf(f);
        return typeMapping.map(mapping -> Optional.ofNullable(log.getFieldValue(f))
          .orElse(mapping.defaultValue()))
          .orElse(null);
      })
      .filter(Objects::nonNull)
      .toArray());
  }
}
