package io.github.zeus.batch.tool

import java.io.FileOutputStream
import java.util.Properties

import com.google.protobuf.CodedOutputStream
import io.github.zeus.batch.{Row, TableOutputStreamBuilder}
import io.github.zeus.rpc.ColumnType._
import io.github.zeus.rpc._
import TableOutputStreamBuilder._
import io.github.zeus.batch.format.blizard.BlizardSegmentOutput._

object BlizardSegmentGenerator {
  def main(args: Array[String]): Unit = {
    val boolColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(BOOL)
      .setId(1)
      .setName("bool")
      .build()

    val int8ColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(INT8)
      .setId(2)
      .setName("int8")
      .build()

    val int16ColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(INT16)
      .setId(3)
      .setName("int16")
      .build()

    val int32ColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(INT32)
      .setId(4)
      .setName("int32")
      .build()

    val int64ColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(INT64)
      .setId(5)
      .setName("int64")
      .build()

    val float4ColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(FLOAT4)
      .setId(6)
      .setName("float4")
      .build()

    val float8ColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(FLOAT8)
      .setId(7)
      .setName("float8")
      .build()

    val timestampColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(TIMESTAMP)
      .setId(8)
      .setName("timestamp")
      .build()

    val stringColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(STRING)
      .setId(9)
      .setName("string")
      .build()

    val tableSchema = ZeusTableSchema.newBuilder()
      .setFormat("blizard")
      .setId(1)
      .setName("test-table")
      .putColumns(1, boolColumnSchema)
      .putColumns(2, int8ColumnSchema)
      .putColumns(3, int16ColumnSchema)
      .putColumns(4, int32ColumnSchema)
      .putColumns(5, int64ColumnSchema)
      .putColumns(6, float4ColumnSchema)
      .putColumns(7, float8ColumnSchema)
      .putColumns(8, timestampColumnSchema)
      .putColumns(9, stringColumnSchema)
      .build()


    val dbSchema = ZeusDBSchema.newBuilder()
      .setId(1)
      .setName("test-db")
      .setVersion(1)
      .putTables(1, tableSchema)
      .build()

    val catalog = ZeusCatalog.newBuilder()
      .addDbSchemas(dbSchema)
      .build()

    val schemaOutput = new FileOutputStream("/home/liurenjie-sal/Downloads/test/test.schema")
    val codedOutput = CodedOutputStream.newInstance(schemaOutput)
    catalog.writeTo(codedOutput)
    codedOutput.flush()
    schemaOutput.flush()
    schemaOutput.close()


    val props = new Properties()
    props.put(ConfigKeySegmentName.key, "/home/liurenjie-sal/Downloads/test/test")
    props.put(ConfigMaxBlockRowNum.key, "2")


    val row1 = Map[Int, Any](
      1 -> true,
      2 -> 1.toByte,
      3 -> 1.toShort,
      4 -> 1,
      5 -> 1L,
      6 -> 1.0f,
      7 -> 1.0,
      8 -> 1L,
      9 -> "1")

    val row2 = Map[Int, Any](
      1 -> false,
      2 -> 2.toByte,
      3 -> 2.toShort,
      4 -> 2,
      5 -> 2L,
      6 -> 2.0f,
      7 -> 2.0,
      8 -> 2L,
      9 -> "2")

    val tableOutputBuilder = TableOutputStreamBuilder(tableSchema, props)

    val tableOutput = tableOutputBuilder.build
    tableOutput.write(new Row(row1))
    tableOutput.write(new Row(row2))
    tableOutput.close()
  }
}
