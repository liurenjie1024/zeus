package io.github.zeus.tool.thrift

import java.util.Optional

import io.github.zeus.rpc.ColumnType
import org.apache.spark.sql.types.{DataType, DataTypes}
import org.apache.thrift.protocol.TType

case class ThriftDataTypeMapping(thriftType: Byte, 
  zeusType: ColumnType, defaultValue: AnyRef)

case class SparkDataTypeMapping(sparkType: DataType, zeusType: ColumnType, defaultValue: AnyRef)


object DataTypeMappings {
  private val ThriftTypeMappings = List(
    ThriftDataTypeMapping(TType.BOOL, ColumnType.BOOL, false),
    ThriftDataTypeMapping(TType.BYTE, ColumnType.INT8, 0.toByte),
    ThriftDataTypeMapping(TType.I16, ColumnType.INT16, 0.toShort),
    ThriftDataTypeMapping(TType.I32, ColumnType.INT32, 0),
    ThriftDataTypeMapping(TType.I64, ColumnType.INT64, 0L),
    ThriftDataTypeMapping(TType.DOUBLE, ColumnType.FLOAT8, 0.0),
    ThriftDataTypeMapping(TType.STRING, ColumnType.STRING, "")
  )
  
  private val SparkTypeMappings = List(
    SparkDataTypeMapping(DataTypes.BooleanType, ColumnType.BOOL, false),
    SparkDataTypeMapping(DataTypes.ByteType, ColumnType.INT8, 0.toByte),
    SparkDataTypeMapping(DataTypes.ShortType, ColumnType.INT16, 0.toShort),
    SparkDataTypeMapping(DataTypes.IntegerType, ColumnType.INT32, 0),
    SparkDataTypeMapping(DataTypes.LongType, ColumnType.INT64, 0L),
    SparkDataTypeMapping(DataTypes.FloatType, ColumnType.FLOAT4, 0.0f),
    SparkDataTypeMapping(DataTypes.DoubleType, ColumnType.FLOAT8, 0.0),
    SparkDataTypeMapping(DataTypes.StringType, ColumnType.STRING, "")
  )

  private val Thrift2Zeus = ThriftTypeMappings.map(m => (m.thriftType, m))
    .toMap

  private val Zeus2Thrift = ThriftTypeMappings.map(m => (m.zeusType, m))
    .toMap

  private val Spark2Zeus = SparkTypeMappings.map(m => (m.sparkType, m))
    .toMap

  private val Zeus2Spark = SparkTypeMappings.map(m => (m.zeusType, m))
    .toMap

  def thriftDataTypeMappingOf(thriftType: Byte): Optional[ThriftDataTypeMapping] = {
    Thrift2Zeus.get(thriftType) match {
      case Some(m) => Optional.of(m)
      case None => Optional.empty()
    }
  }

  def thriftDataTypeMappingOf(zeusType: ColumnType): Optional[ThriftDataTypeMapping] = {
    Zeus2Thrift.get(zeusType) match {
      case Some(m) => Optional.of(m)
      case None => Optional.empty()
    }
  }

  def sparkDataTypeMappingOf(sparkType: DataType): Optional[SparkDataTypeMapping] = {
    Spark2Zeus.get(sparkType) match {
      case Some(m) => Optional.of(m)
      case None => Optional.empty()
    }
  }

  def sparkDataTypeMappingOf(zeusType: ColumnType): Optional[SparkDataTypeMapping] = {
    Zeus2Spark.get(zeusType) match {
      case Some(m) => Optional.of(m)
      case None => Optional.empty()
    }
  }
}
