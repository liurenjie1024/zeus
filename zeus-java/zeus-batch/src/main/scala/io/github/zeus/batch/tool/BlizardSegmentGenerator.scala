package io.github.zeus.batch.tool

import java.io.FileOutputStream
import java.util.Properties

import com.google.protobuf.CodedOutputStream
import io.github.zeus.batch.{Row, TableOutputStreamBuilder}
import io.github.zeus.rpc._

object BlizardSegmentGenerator {
  def main(args: Array[String]): Unit = {
    val boolColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(ColumnType.BOOL)
      .setId(1)
      .setName("bool")
      .build()

    val byteColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(ColumnType.INT8)
      .setId(2)
      .setName("byte")
      .build()

    val floatColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(ColumnType.FLOAT4)
      .setId(3)
      .setName("float")
      .build()

    val intColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(ColumnType.INT32)
      .setId(4)
      .setName("int")
      .build()

    val longColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(ColumnType.INT64)
      .setId(5)
      .setName("long")
      .build()

    val stringColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(ColumnType.STRING)
      .setId(6)
      .setName("string")
      .build()

    val tableSchema = ZeusTableSchema.newBuilder()
      .setFormat("blizard")
      .setId(1)
      .setName("table")
      .putColumns(1, boolColumnSchema)
      .putColumns(2, byteColumnSchema)
      .putColumns(3, floatColumnSchema)
      .putColumns(4, intColumnSchema)
      .putColumns(5, longColumnSchema)
      .putColumns(6, stringColumnSchema)
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

    val schemaOutput = new FileOutputStream("/home/liurenjie-sal/Downloads/test.schema")
    val codedOutput = CodedOutputStream.newInstance(schemaOutput)
    catalog.writeTo(codedOutput)
    codedOutput.flush()
    schemaOutput.flush()
    schemaOutput.close()


    val props = new Properties()
    props.put("output.type", "file")
    props.put("output.file", "/home/liurenjie-sal/Downloads/test.data")
    props.put("blizard.block.row.num", "3")


    val row1 = Map[Int, Any](1 -> true,
      2 -> 1.toByte,
      3 -> 1.0f,
      4 -> 1,
      5 -> 1L,
      6 -> "1")

    val row2 = Map[Int, Any](1 -> false,
      2 -> 2.toByte,
      3 -> 2.0f,
      4 -> 2,
      5 -> 2L,
      6 -> "12")

    val row3 = Map[Int, Any](1 -> true,
      2 -> 3.toByte,
      3 -> 3.0f,
      4 -> 3,
      5 -> 3L,
      6 -> "123")

    val tableOutputBuilder = TableOutputStreamBuilder(tableSchema, props)

    val tableOutput = tableOutputBuilder.build
    tableOutput.write(new Row(row1))
    tableOutput.write(new Row(row2))
    tableOutput.write(new Row(row3))
    tableOutput.close()
  }
}
