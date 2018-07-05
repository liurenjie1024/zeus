package io.github.zeus.batch.format.parquet

import org.apache.spark.sql.SparkSession

object ParquetReader {
  def main(args: Array[String]): Unit = {
    val spark = SparkSession.builder()
      .appName("zeus-parquet-generator")
      .getOrCreate()

    val data = spark.read.parquet("rt-parquet/*.parquet")
        .count()
    println(s"Line number is ${data}")

    spark.close()
  }
}
