package io.github.zeus.tool.thrift;

import com.mediav.realtime.log.RealtimeLog;
import io.github.zeus.batch.Row;
import io.github.zeus.batch.RowBuilder;
import io.github.zeus.rpc.ColumnType;
import io.github.zeus.rpc.ZeusColumnSchema;
import io.github.zeus.rpc.ZeusTableSchema;
import org.apache.thrift.TBase;
import org.apache.thrift.TFieldIdEnum;
import org.apache.thrift.meta_data.FieldMetaData;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.EnumSet;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

public class ThriftConverter<T extends TBase<T, F>, F extends Enum<F> & TFieldIdEnum> {
  private static final Logger LOGGER = LoggerFactory.getLogger(ThriftConverter.class);
  private final Class<T> klass;


  private EnumSet<F> fields;
  private Map<F, FieldMetaData> filedMetaDataMap;
  private ZeusTableSchema partialSchema;

  public ThriftConverter(Class<T> klass) {
    Objects.requireNonNull(klass, "Thrift class can't be null");
    this.klass = klass;
  }

  @SuppressWarnings(value = "unchecked")
  private void initZeusSchema() {
    if (partialSchema != null) {
      return;
    }
    try {
      ZeusTableSchema.Builder schemaBuilder =  ZeusTableSchema.newBuilder();
      Class<F> fieldEnumClass = (Class<F>) Class.forName(klass.getName() + "$_Fields");
      filedMetaDataMap = (Map<F, FieldMetaData>) klass.getField("metaDataMap").get(null);
      fields = EnumSet.allOf(fieldEnumClass);

      fields.forEach(field -> {
          Optional<ColumnType> columnType = ThriftDataType.columnTypeOf(
            filedMetaDataMap.get(field).valueMetaData.type);
          if (columnType.isPresent()) {
            ZeusColumnSchema columnSchema =  ZeusColumnSchema.newBuilder()
              .setId(field.getThriftFieldId())
              .setName(field.getFieldName())
              .setColumnType(columnType.get())
              .build();

            schemaBuilder.putColumns(columnSchema.getId(), columnSchema);
          } else {
            LOGGER.warn("Unable to find column type for field({}) of {}", field.getFieldName(), klass);
          }
        });

      partialSchema = schemaBuilder.build();
    } catch (Exception e) {
      throw new IllegalArgumentException(e);
    }
  }

  public ZeusTableSchema createSchema() {
    initZeusSchema();
    return partialSchema;
  }

  public Row createRow(T log) {
    initZeusSchema();

    RowBuilder builder = new RowBuilder();

    fields.forEach(f -> {
      thriftDataTypeOf(f)
        .map(t -> Optional.ofNullable(log.getFieldValue(f)).orElseGet(t::getDefaultValue))
        .ifPresent(v -> builder.put(f.getThriftFieldId(), v));
    });

    return builder.build();
  }

  private Optional<ThriftDataType> thriftDataTypeOf(F field) {
    return ThriftDataType.thriftDataTypeOf(filedMetaDataMap.get(field).valueMetaData.type);
  }

  public static void main(String[] args) {
    ThriftConverter<RealtimeLog, RealtimeLog._Fields> converter = new ThriftConverter<>(RealtimeLog.class);
    ZeusTableSchema schema = converter.createSchema();
    System.out.println(schema);
  }
}
