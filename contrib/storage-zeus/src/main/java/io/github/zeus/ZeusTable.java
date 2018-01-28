package io.github.zeus;

import io.github.zeus.rpc.FieldType;
import io.github.zeus.rpc.ZeusColumnSchema;
import io.github.zeus.rpc.ZeusTableSchema;
import org.apache.calcite.rel.type.RelDataType;
import org.apache.calcite.rel.type.RelDataTypeFactory;
import org.apache.calcite.sql.type.SqlTypeName;
import org.apache.drill.exec.planner.logical.DynamicDrillTable;
import org.apache.drill.exec.store.StoragePlugin;

import java.util.List;
import java.util.stream.Collectors;

/**
 * Created by liurenjie on 24/01/2018.
 */
public class ZeusTable extends DynamicDrillTable {
  private final ZeusTableSchema tableSchema;

  public ZeusTable(StoragePlugin plugin, String storageEngineName, Object selection, ZeusTableSchema tableSchema) {
    super(plugin, storageEngineName, selection);
    this.tableSchema = tableSchema;
  }

  @Override
  public RelDataType getRowType(final RelDataTypeFactory typeFactory) {
    List<RelDataType> dataTypes = this.tableSchema.getFieldsList().stream()
      .map(f -> toDrillType(typeFactory, f.getFieldType()))
      .map(t -> typeFactory.createTypeWithNullability(t, false))
      .collect(Collectors.toList());

    List<String> names = this.tableSchema.getFieldsList().stream()
      .map(ZeusColumnSchema::getName)
      .collect(Collectors.toList());

    return typeFactory.createStructType(dataTypes, names);
  }

  private static RelDataType toDrillType(RelDataTypeFactory typeFactory, FieldType fieldType) {
    switch (fieldType) {
      case STRING:
        return typeFactory.createSqlType(SqlTypeName.VARCHAR);
      case FLOAT:
        return typeFactory.createSqlType(SqlTypeName.FLOAT);
      case INT32:
        return typeFactory.createSqlType(SqlTypeName.INTEGER);
      case INT64:
        return typeFactory.createSqlType(SqlTypeName.BIGINT);
      case BOOL:
        return typeFactory.createSqlType(SqlTypeName.BOOLEAN);
      case TIMESTAMP:
        return typeFactory.createSqlType(SqlTypeName.TIMESTAMP);
      default:
        throw new IllegalArgumentException("Unrecognized type: " + fieldType.name());
    }
  }
}
