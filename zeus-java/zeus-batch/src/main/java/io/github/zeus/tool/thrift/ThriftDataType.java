package io.github.zeus.tool.thrift;

import io.github.zeus.rpc.ColumnType;
import org.apache.thrift.protocol.TType;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;

public enum ThriftDataType {
  Bool(TType.BOOL, ColumnType.BOOL, false),
  Int8(TType.BYTE, ColumnType.INT8, (byte)0),
  Int16(TType.I16, ColumnType.INT16, (short)0),
  Int32(TType.I32, ColumnType.INT32, 0),
  Int64(TType.I64, ColumnType.INT64, 0L),
  Float8(TType.DOUBLE, ColumnType.FLOAT8, 0.0),
  UTF8(TType.STRING, ColumnType.STRING, "");

  private static final Map<Byte, ThriftDataType> Thrift2ColumnType = new HashMap<>();

  static {
    for (ThriftDataType thriftDataType : ThriftDataType.values()) {
      Thrift2ColumnType.put(thriftDataType.ttype, thriftDataType);
    }
  }

  private final byte ttype;
  private final ColumnType columnType;
  private final Object defaultValue;

  ThriftDataType(byte ttype, ColumnType columnType, Object defaultValue) {
    this.ttype = ttype;
    this.columnType = columnType;
    this.defaultValue = defaultValue;
  }

  public Object getDefaultValue() {
    return this.defaultValue;
  }

  public ColumnType getColumnType() {
    return this.columnType;
  }

  public static Optional<ThriftDataType> thriftDataTypeOf(byte ttype) {
    return Optional.ofNullable(Thrift2ColumnType.get(ttype));
  }

  public static Optional<ColumnType> columnTypeOf(byte ttype) {
    return thriftDataTypeOf(ttype).map(ThriftDataType::getColumnType);
  }
}
