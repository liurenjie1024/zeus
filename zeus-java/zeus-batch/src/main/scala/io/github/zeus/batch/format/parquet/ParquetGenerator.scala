package io.github.zeus.batch.format.parquet

import com.mediav.realtime.log.RealtimeLog
import io.github.zeus.tool.thrift.spark.ThriftDataFrameBuilder
import org.apache.spark.sql.SparkSession

object ParquetGenerator {
  def main(args: Array[String]) = {
    val spark = SparkSession.builder()
      .appName("zeus-parquet-generator")
      .getOrCreate()

    val df = new ThriftDataFrameBuilder[RealtimeLog, RealtimeLog._Fields]("/mvad/rawlog/dsp-charge/2018-07-03/*/dsp.charge.6.click/*")
      .build(spark)
      .limit(1000)

    println(s"Dataframe size is ${df.count()}")

    df.coalesce(1)
      .write.parquet("rt-parquet")

    spark.close()
  }
}
