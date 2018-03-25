package io.github.zeus.batch

import java.io.OutputStream
import java.util.Properties

import io.github.zeus.batch.TableOutputStreamBuilder._
import io.github.zeus.batch.format.simple.SimpleSegmentOutputStream
import io.github.zeus.rpc.ZeusTableSchema

case class TableOutputStreamBuilder(tableSchema: ZeusTableSchema, config: Properties) {
  private val output = OutputConfigOption.get(config)

  def build: TableOutputStream = {
    buildTableOutputStream(this)
  }

  def getOutput: OutputStream = output
}

object TableOutputStreamBuilder {
  type TableOutputStreamFactory = TableOutputStreamBuilder => TableOutputStream
  val FormatSimple = "simple"
  private val FormatRegistry: Map[String, TableOutputStreamFactory] = Map(
    FormatSimple -> buildSimpleTableOutputStream)



  private def buildSimpleTableOutputStream(builder: TableOutputStreamBuilder): TableOutputStream = {
    new SimpleSegmentOutputStream(builder)
  }

  def buildTableOutputStream(builder: TableOutputStreamBuilder): TableOutputStream = {
    FormatRegistry(builder.tableSchema.getFormat)(builder)
  }
}

