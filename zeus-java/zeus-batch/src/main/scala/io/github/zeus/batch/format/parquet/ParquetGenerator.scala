package io.github.zeus.batch.format.parquet


import com.google.protobuf.CodedOutputStream
import com.mediav.realtime.log.RealtimeLog
import io.github.zeus.rpc.{ZeusCatalog, ZeusDBSchema, ZeusTableSchema}
import io.github.zeus.tool.thrift.ThriftConverter
import io.github.zeus.tool.thrift.spark.ThriftDataFrameBuilder
import org.apache.hadoop.conf.Configuration
import org.apache.hadoop.fs.{FileSystem, Path}
import org.apache.spark.sql.SparkSession

object ParquetGenerator {
  val folder = "rt-parquet"
  def main(args: Array[String]) = {

  }

  def storeSchema: Unit = {
    val config = new Configuration()
    val fs = FileSystem.get(config)

    val schemaPath = new Path(s"$folder/logs.schema")
    val schemaOutput = fs.create(schemaPath, true)

    val catalogOuptput = CodedOutputStream.newInstance(schemaOutput)

    try {
      getCatalog.writeTo(catalogOuptput)
      catalogOuptput.flush()
    } finally {
      schemaOutput.close()
    }
  }

  def getCatalog: ZeusCatalog = {
    val converter = new ThriftConverter[RealtimeLog, RealtimeLog._Fields](classOf[RealtimeLog])
    val tableSchema = ZeusTableSchema.newBuilder(converter.createSchema())
      .setId(1)
      .setName("realtimelog")
      .setFormat("blizard")
      .build()

    val dbSchema = ZeusDBSchema.newBuilder()
      .setId(1)
      .setName("logs")
      .putTables(tableSchema.getId, tableSchema)
      .build()

    ZeusCatalog.newBuilder()
      .addDbSchemas(dbSchema)
      .build()
  }

  def storeData: Unit = {
    val spark = SparkSession.builder()
      .appName("zeus-parquet-generator")
      .getOrCreate()

    val df = new ThriftDataFrameBuilder[RealtimeLog, RealtimeLog._Fields]("/mvad/rawlog/dsp-charge/2018-07-03/*/dsp.charge.6.click/*")
      .build(spark)
      .limit(1000)

//    println(s"Dataframe size is ${df.count()}")
    df.coalesce(1)
      .write.parquet(s"$folder/1")

    spark.close()
  }
}
