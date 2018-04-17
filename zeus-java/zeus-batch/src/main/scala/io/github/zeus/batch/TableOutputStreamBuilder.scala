package io.github.zeus.batch

import java.nio.channels.WritableByteChannel
import java.util.Properties

import io.github.zeus.batch.TableOutputStreamBuilder._
import io.github.zeus.batch.format.blizard.BlizardSegmentOutputStream
import io.github.zeus.rpc.ZeusTableSchema

case class TableOutputStreamBuilder(tableSchema: ZeusTableSchema, config: Properties) {
  def build: TableOutputStream = {
    buildTableOutputStream(this)
  }
}

object TableOutputStreamBuilder {
  val ConfigKeySegmentName = "output.segment.name"


  type TableOutputStreamFactory = TableOutputStreamBuilder => TableOutputStream
  val FormatBlizard = "blizard"
  private val FormatRegistry: Map[String, TableOutputStreamFactory] = Map(
    FormatBlizard  -> buildBlizardTableOutputStream)



  private def buildBlizardTableOutputStream(builder: TableOutputStreamBuilder): TableOutputStream = {
    new BlizardSegmentOutputStream(builder)
  }

  def buildTableOutputStream(builder: TableOutputStreamBuilder): TableOutputStream = {
    FormatRegistry(builder.tableSchema.getFormat)(builder)
  }
}

