package io.github.zeus.client.meta;

import java.util.Collections;
import java.util.List;

public class ResultMetadata {
  private final List<ColumnMeta> columnMetaList;

  public ResultMetadata(List<ColumnMeta> columnMetaList) {
    this.columnMetaList = columnMetaList;
  }

  public int getColumnCount() {
    return columnMetaList.size();
  }

  public ColumnMeta getColumnMeta(int index) {
    return columnMetaList.get(index);
  }

  public List<ColumnMeta> getColumnMetaList() {
    return Collections.unmodifiableList(columnMetaList);
  }
}
