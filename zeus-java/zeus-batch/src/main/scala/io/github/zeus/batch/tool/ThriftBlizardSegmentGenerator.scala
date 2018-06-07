package io.github.zeus.batch.tool

import java.io.FileOutputStream
import java.util.Properties

import com.google.protobuf.CodedOutputStream
import com.mediav.realtime.log.RealtimeLog
import io.github.zeus.batch.TableOutputStreamBuilder
import io.github.zeus.batch.TableOutputStreamBuilder.ConfigKeySegmentName
import io.github.zeus.batch.format.blizard.BlizardSegmentOutput.ConfigMaxBlockRowNum
import io.github.zeus.rpc.{ZeusCatalog, ZeusDBSchema, ZeusTableSchema}
import io.github.zeus.tool.thrift.ThriftConverter
import org.apache.thrift.TDeserializer
import org.apache.thrift.protocol.TJSONProtocol
import org.slf4j.LoggerFactory

import scala.io.Source

object ThriftBlizardSegmentGenerator {
  val LOGGER = LoggerFactory.getLogger("thrift-blizard-segment-generator")
  val prefix = "/home/liurenjie-sal/Downloads/test/"
  val thriftConverter = new ThriftConverter[RealtimeLog, RealtimeLog._Fields](classOf[RealtimeLog])

  def main(args: Array[String]): Unit = {
    writeSchema()
    writeLogs()
  }

  def writeSchema(): Unit = {
    val tableSchema = createTableSchema()

    val zeusDBSchema = ZeusDBSchema.newBuilder()
      .setName("logs")
      .setId(1)
      .putTables(tableSchema.getId, tableSchema)
      .build()

    val catalog = ZeusCatalog.newBuilder()
      .addDbSchemas(zeusDBSchema)
      .build()

    val fileOutputStream = new FileOutputStream(s"${prefix}logs.schema")
    val outputStream = CodedOutputStream.newInstance(fileOutputStream)

    try {
      catalog.writeTo(outputStream)
      outputStream.flush()
    } finally {
      fileOutputStream.close()
    }
  }

  def createTableSchema(): ZeusTableSchema = {
    thriftConverter.createSchema()
      .toBuilder
      .setFormat("blizard")
      .setName("realtimelog")
      .setId(1)
      .build()
  }

  def writeLogs(): Unit = {
    val config = new Properties()
    config.put(ConfigKeySegmentName.key, s"${prefix}realtimelog1")
    config.put(ConfigMaxBlockRowNum.key, "512")

    val tableOutputBuilder = TableOutputStreamBuilder(createTableSchema(), config)
    val tableOutput = tableOutputBuilder.build

    val deserializer = new TDeserializer(new TJSONProtocol.Factory)

    val deserialize = (line: String) => {
      val log = new RealtimeLog()
      deserializer.deserialize(log, line, "utf-8")
      log
    }
    Source.fromFile(s"${prefix}logs1000")
      .getLines()
      .map(deserialize)
      .map(thriftConverter.createRow)
      .foreach(tableOutput.write)

    tableOutput.close()
  }
}
