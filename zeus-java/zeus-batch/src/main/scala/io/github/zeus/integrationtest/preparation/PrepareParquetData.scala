package io.github.zeus.integrationtest.preparation

import com.google.protobuf.CodedOutputStream
import com.mediav.realtime.log.RealtimeLog
import io.github.zeus.rpc.{ZeusCatalog, ZeusDBSchema, ZeusTableSchema}
import io.github.zeus.tool.thrift.ThriftConverter
import io.github.zeus.tool.thrift.spark.ThriftDataFrameBuilder
import io.github.zeus.utils.Utils
import org.apache.hadoop.conf.Configuration
import org.apache.hadoop.fs.{FileSystem, Path}
import org.apache.spark.sql.SparkSession
import org.slf4j.LoggerFactory

object PrepareParquetData {
  case class Config(sourcePath: String = null, destPath: String = null,
    partitionLimit: Int = 10000, partitionNum: Int = 8)
  val LOG = LoggerFactory.getLogger(PrepareParquetData.getClass.getName)

  var execConfig: Config = _

  def main(args: Array[String]): Unit = {
    execConfig = new scopt.OptionParser[Config]("PreparingParquet") {
      head("PreparingParquet", "0.1")

      opt[String]('s', "source")
        .action((source, c) => c.copy(sourcePath = source))
        .required()

      opt[String]('d', "dest")
        .action((dest, c) => c.copy(destPath = dest))
        .required()

      opt[Int]('p', "partitionNum")
        .action((partitionNum, c) => c.copy(partitionNum = partitionNum))
        .optional()

      opt[Int]('n', "partitionLimit")
        .action((partitionLimit, c) => c.copy(partitionLimit = partitionLimit))
        .optional()

    }.parse(args, Config()) match {
      case Some(c) => c
      case None =>
        System.exit(1)
        null
    }

    storeSchema
    storeData
  }

  def storeSchema: Unit = {
    val config = new Configuration()
    val fs = FileSystem.get(config)

    val path = new Path(s"${execConfig.destPath}")
    if (!Utils.isEmptyDir(path, fs)) {
      throw new IllegalStateException(s"${path.toUri} is not empty")
    }
    fs.mkdirs(path)
    val schemaPath = new Path(s"${execConfig.destPath}/logs.schema")
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

    new ThriftDataFrameBuilder[RealtimeLog, RealtimeLog._Fields](execConfig.sourcePath)
      .build(spark)
      .limit(execConfig.partitionNum * execConfig.partitionLimit)
      .repartition(execConfig.partitionNum)
      .write.parquet(s"${execConfig.destPath}/1")

    spark.close()
  }
}
