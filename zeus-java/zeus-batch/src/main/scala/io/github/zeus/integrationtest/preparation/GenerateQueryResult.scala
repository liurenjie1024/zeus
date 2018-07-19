package io.github.zeus.integrationtest.preparation

import io.github.zeus.integrationtest.{SqlElement, Utils}
import org.apache.hadoop.conf.Configuration
import org.apache.hadoop.fs.{FileSystem, Path}
import org.apache.spark.sql.SparkSession
import org.slf4j.{Logger, LoggerFactory}
import scopt.OptionParser

object GenerateQueryResult {
  val LOG: Logger = LoggerFactory.getLogger(GenerateQueryResult.getClass.getName)
  case class Config(sqlPath: String = null,
    parquetPath: String = null,
    outputPath: String = null,
    viewName: String = null)

  var execConfig: Config = _
  var fs: FileSystem = _

  def main(args: Array[String]): Unit = {
    execConfig = new OptionParser[Config]("generate-query-result") {
      head("generate-query-result", "0.1.0")

      opt[String]('s', "sql")
        .action((s, c) => c.copy(sqlPath = s))
        .required()

      opt[String]('p', "parquet")
        .action((p, c) => c.copy(parquetPath = p))
        .required()

      opt[String]('o', "output")
        .action((o, c) => c.copy(outputPath = o))
        .required()

      opt[String]('n', "viewName")
        .action((o, c) => c.copy(outputPath = o))
        .required()
    }.parse(args, Config()) match {
      case Some(t) => t
      case None =>
        System.exit(1)
        null
    }

    run
  }

  def run: Unit = {
    checkAndMkOutputDir
    val sqls = SqlElement.loadFile(execConfig.sqlPath)

    val spark = SparkSession.builder()
      .appName("zeus-query-result-generator")
      .getOrCreate()

    val df = spark.read.parquet(execConfig.parquetPath)

    df.createTempView(execConfig.viewName)

    for (sql <- sqls) {
      val sqlOutputPath = s"${execConfig.outputPath}/${sql.name}"
      fs.mkdirs(new Path(sqlOutputPath))

      spark.sql(sql.sql)
        .coalesce(1)
        .write.json(sqlOutputPath)
    }
  }

  def checkAndMkOutputDir: Unit = {
    val config = new Configuration()
    fs = FileSystem.get(config)

    val outputPath = new Path(execConfig.outputPath)
    if (!Utils.isEmptyDir(outputPath, fs)) {
      throw new IllegalArgumentException(s"${outputPath} is not empty!")
    }

    fs.mkdirs(outputPath)
  }
}
