package io.github.zeus.batch.format.parquet

import com.mediav.data.log.unitedlog.UnitedEvent
import io.github.zeus.tool.thrift.spark.ThriftDataFrameBuilder
import org.apache.spark.sql.SparkSession

object ParquetGenerator {
  def main(args: Array[String]) = {
    val spark = SparkSession.builder()
      .appName("zeus-parquet-generator")
      .getOrCreate()

    new ThriftDataFrameBuilder[UnitedEvent, UnitedEvent._Fields]("/mvad/rawlog/dsp-charge/2018-07-03/*/dsp.charge.6.click/*")
      .build(spark)
      .limit(1000)
      .coalesce(1)
      .write.parquet("rt-parquet")

    spark.close()
  }
}
