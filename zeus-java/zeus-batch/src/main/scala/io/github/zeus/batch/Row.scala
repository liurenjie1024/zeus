package io.github.zeus.batch

import scala.collection.mutable

/**
  * The key of columns is column id, value is column value.
 *
  * @param columns
  */
class Row(private val columns: Map[Int, Any]) {
  def getColumnValue(columnId: Int): Option[Any] = columns.get(columnId)
}

class RowBuilder {
  private val columns: mutable.Map[Int, Any] = mutable.Map()

  def put(columnId: Int, value: Any): RowBuilder = {
    require(value != null)
    columns(columnId) = value
    this
  }

  def build: Row = {
    new Row(columns.toMap)
  }
}
