package io.github.zeus.tool.thrift.spark

import com.mediav.elephantbird.mapreduce.input.MultiInputFormat
import com.twitter.elephantbird.mapreduce.io.BinaryWritable
import org.apache.hadoop.io.LongWritable
import org.apache.hadoop.mapreduce.Job
import org.apache.spark.rdd.RDD
import org.apache.spark.sql.{DataFrame, SparkSession}
import org.apache.thrift.{TBase, TFieldIdEnum}

import scala.reflect.ClassTag

class ThriftDataFrameBuilder[T <: TBase[T, F], F <: Enum[F] with TFieldIdEnum](path: String)(implicit c: ClassTag[T])
  extends Serializable {
  private val sparkThriftConverter = new SparkThriftConverter[T, F](c.runtimeClass.asInstanceOf[Class[T]])

  def build(spark: SparkSession): DataFrame = {
    val rowRDD = buildRDD(spark)
      .filter(_ != null)
      .map(sparkThriftConverter.createRow)

    spark.createDataFrame(rowRDD, sparkThriftConverter.getSchema)
  }

  def buildRDD(spark: SparkSession): RDD[T] = {
    val jobConf = Job.getInstance().getConfiguration
    MultiInputFormat.setClassConf(c.runtimeClass.asInstanceOf[Class[T]], jobConf)
    Job.getInstance().setInputFormatClass(classOf[MultiInputFormat[T]])

    spark.sparkContext.newAPIHadoopFile[LongWritable, BinaryWritable[T], MultiInputFormat[T]](
      path,
      classOf[MultiInputFormat[T]],
      classOf[LongWritable],
      classOf[BinaryWritable[T]], jobConf)
      .map(_._2.get())
  }
}