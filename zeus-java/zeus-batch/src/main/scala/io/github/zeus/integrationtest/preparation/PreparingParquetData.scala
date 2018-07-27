package io.github.zeus.integrationtest.preparation


import com.google.protobuf.CodedOutputStream
import com.mediav.realtime.log.RealtimeLog
import io.github.zeus.integrationtest.Command
import io.github.zeus.rpc.{ZeusCatalog, ZeusDBSchema, ZeusTableSchema}
import io.github.zeus.tool.thrift.ThriftConverter
import io.github.zeus.tool.thrift.spark.ThriftDataFrameBuilder
import io.github.zeus.utils.Utils
import org.apache.hadoop.conf.Configuration
import org.apache.hadoop.fs.{FileSystem, Path}
import org.apache.spark.sql.SparkSession
import org.rogach.scallop.{ScallopOption, Subcommand}
import org.slf4j.LoggerFactory

class PreparingParquetData(execConfig: PreparingParquetDataArgs) extends Command {
  val LOG = LoggerFactory.getLogger(classOf[PreparingParquetData])

  override def run(): Unit = {
    storeSchema
    storeData
  }

  def storeSchema: Unit = {
    val config = new Configuration()
    val fs = FileSystem.get(config)

    val path = new Path(s"${execConfig.destPath()}")
    if (!Utils.isEmptyDir(path, fs)) {
      throw new IllegalStateException(s"${path.toUri} is not empty")
    }
    fs.mkdirs(path)
    val schemaPath = new Path(s"${execConfig.destPath()}/logs.schema")
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

    import spark.implicits._

    val df = new ThriftDataFrameBuilder[RealtimeLog, RealtimeLog._Fields](execConfig.sourcePath())
      .build(spark)
      .limit(execConfig.totalNum.get)
      .repartition(execConfig.partitionNum())
      .sortWithinPartitions($"timestamp".desc)
      .write

    execConfig.parquetBlockSize
      .toOption
      .foreach(x => df.option("parquet.block.size", x))

    df.parquet(s"${execConfig.destPath()}/1")
    spark.close()
  }
}


class PreparingParquetDataArgs extends Subcommand("preparing-parquet") {
  val sourcePath: ScallopOption[String] = opt[String]("sourcePath", 's', required = true)
  val destPath: ScallopOption[String] = opt[String]("destPath", 'd', required = true)
  val partitionLimit: ScallopOption[Int] = opt[Int]("partitionLimit", 'l', default = Some(1000), required = false)
  val partitionNum: ScallopOption[Int] = opt[Int]("partitionNum", 'p', default = Some(8), required = false)
  val parquetBlockSize: ScallopOption[Long] = opt[Long]("parquetBlockSize", 'b')

  def totalNum: Option[Int] = {
    (partitionLimit.toOption, partitionNum.toOption) match {
      case (Some(limit), Some(num)) => Some(num * limit)
      case _ => None
    }
  }
}
