package io.github.zeus.batch

import java.io.OutputStream
import java.util.Properties

import io.github.zeus.rpc.ZeusTableSchema
import TableOutputStreamBuilder._

case class TableOutputStreamBuilder(tableSchema: ZeusTableSchema, config: Properties) {
  def build: TableOutputStream = {
    buildTableOutputStream(this)
  }

  def getOutputStream: OutputStream = ???
}

object TableOutputStreamBuilder {
  type TableOutputStreamFactory = TableOutputStreamBuilder => TableOutputStream
  val FormatSimple = "simple"
  private val FormatRegistry: Map[String, TableOutputStreamFactory] = Map(
    FormatSimple -> buildSimpleTableOutputStream)

  private def buildSimpleTableOutputStream(builder: TableOutputStreamBuilder): TableOutputStream = {
    ???
  }

  def buildTableOutputStream(builder: TableOutputStreamBuilder): TableOutputStream = {
    FormatRegistry(builder.tableSchema.getFormat)(builder)
  }
}