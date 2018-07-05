package io.github.zeus.batch.format.parquet

import org.apache.spark.sql.SparkSession

object ParquetReader {
  def main(args: Array[String]): Unit = {
    val spark = SparkSession.builder()
      .appName("zeus-parquet-generator")
      .getOrCreate()

    val lines = spark.read.parquet("rt-parquet")
        .count()
    println(s"Line number is ${lines}")

    spark.close()
  }
}
