package io.github.zeus.integrationtest.verify

import java.io.PrintWriter
import java.sql.ResultSet

case class JDBCResultSet(columnNames: List[String],
  columnTypeIds: List[Int],
  columnTypeNames: List[String],
  results: List[List[AnyRef]]) {
  def output(writer: PrintWriter): Unit = {
    writer.println(s"column names: ${columnNames}, column type names: ${columnTypeNames}")
    results.foreach { row =>
      writer.println(row.mkString(","))
    }
  }
}

object JDBCResultSet {
  def fromResultSet(rs: ResultSet): JDBCResultSet = {
    val metadata = rs.getMetaData
    val columnCount = metadata.getColumnCount

    val columnNames = (1 to columnCount)
      .map(metadata.getColumnName)
      .toList
    val columnTypeIds = (1 to columnCount)
      .map(metadata.getColumnType)
      .toList
    val columnTypeNames = (1 to columnCount)
      .map(metadata.getColumnTypeName)
      .toList

    val results = List.newBuilder[List[AnyRef]]
    while (rs.next()) {
      results += (1 to columnCount).map(rs.getObject).toList
    }

    JDBCResultSet(columnNames, columnTypeIds, columnTypeNames,
      results.result())
  }

  def compareUnordered(expected: JDBCResultSet, actual: JDBCResultSet): Boolean = {
    if (!compareMetadata(expected, actual)) {
      return false
    }

    val expectedResults = expected.results.groupBy(v => v).mapValues(_.size)
    val actualResults = actual.results.groupBy(v => v).mapValues(_.size)

    expectedResults == actualResults
  }

  def compareOrdered(expected: JDBCResultSet, actual: JDBCResultSet): Boolean = {
    if (!compareMetadata(expected, actual)) {
      return false
    }

    expected.results == actual.results
  }

  def compareMetadata(expected: JDBCResultSet, actual: JDBCResultSet): Boolean = {
    (expected.columnNames == actual.columnNames) &&
      (expected.columnTypeIds == actual.columnTypeIds) &&
      (expected.columnTypeNames == actual.columnTypeNames)
  }
}

