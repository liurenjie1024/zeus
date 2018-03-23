package io.github.zeus.batch

/**
  * The key of columns is column id, value is column value.
  * @param columns
  */
class Row(private val columns: Map[Int, Any]) {
  def getColumnValue(columnId: Int): Option[Any] = columns.get(columnId)
}
