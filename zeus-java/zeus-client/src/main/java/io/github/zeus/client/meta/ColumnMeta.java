package io.github.zeus.client.meta;

import io.github.zeus.rpc.ColumnType;
import io.github.zeus.rpc.Expression;
import io.github.zeus.rpc.ProjectNode;

public class ColumnMeta {
  private final String columnName;
  private final ColumnType columnType;


  public ColumnMeta(String columnName, ColumnType columnType) {
    this.columnName = columnName;
    this.columnType = columnType;
  }

  public String getColumnName() {
    return columnName;
  }

  public ColumnType getColumnType() {
    return columnType;
  }

  public static ColumnMeta from(ProjectNode.ProjectItem projectItem) {
    return new ColumnMeta(projectItem.getAlias(),
        projectItem.getExpression().getFieldType());
  }

  public static ColumnMeta from(Expression expression) {
    return new ColumnMeta(expression.getAlias(), expression.getFieldType());
  }
}
