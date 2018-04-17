package io.github.zeus.batch

import java.nio.channels.{Channels, FileChannel, WritableByteChannel}
import java.nio.file._
import java.util.Properties

import io.github.zeus.batch.TableOutputStreamBuilder._
import io.github.zeus.batch.format.blizard.BlizardSegmentOutputStream
import io.github.zeus.batch.util.ConfigOption
import io.github.zeus.rpc.ZeusTableSchema
import StandardOpenOption.{CREATE_NEW, WRITE}

case class TableOutputStreamBuilder(tableSchema: ZeusTableSchema, config: Properties) {
  def build: TableOutputStream = {
    buildTableOutputStream(this)
  }
}

object TableOutputStreamBuilder {
  val ConfigKeySegmentName: ConfigOption[String] = ConfigOption[String]("output.segment.name")


  type TableOutputStreamFactory = TableOutputStreamBuilder => TableOutputStream
  val FormatBlizard = "blizard"
  private val FormatRegistry: Map[String, TableOutputStreamFactory] = Map(
    FormatBlizard  -> buildBlizardTableOutputStream)



  private def buildBlizardTableOutputStream(builder: TableOutputStreamBuilder): TableOutputStream = {
    val prefix = ConfigKeySegmentName.get(builder.config).get

    val openOptions = Array(CREATE_NEW, WRITE)
    val indexOutput = Channels.newOutputStream(FileChannel.open(Paths.get(s"$prefix.idx"), openOptions:_*))

    val dataOutput = Channels.newOutputStream(FileChannel.open(Paths.get(s"$prefix.bin"), openOptions:_*))

    new BlizardSegmentOutputStream(builder.config, builder.tableSchema, indexOutput, dataOutput)
  }

  def buildTableOutputStream(builder: TableOutputStreamBuilder): TableOutputStream = {
    FormatRegistry(builder.tableSchema.getFormat)(builder)
  }
}

