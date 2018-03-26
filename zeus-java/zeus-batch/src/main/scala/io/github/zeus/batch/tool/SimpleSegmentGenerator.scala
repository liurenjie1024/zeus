package io.github.zeus.batch.tool

import java.io.FileOutputStream
import java.util.Properties

import com.google.protobuf.CodedOutputStream
import io.github.zeus.batch.{Row, TableOutputStreamBuilder}
import io.github.zeus.rpc.{FieldType, ZeusColumnSchema, ZeusTableSchema}

object SimpleSegmentGenerator {
  def main(args: Array[String]): Unit = {
    val boolColumnSchema = ZeusColumnSchema.newBuilder()
      .setFieldType(FieldType.BOOL)
      .setId(1)
      .setName("bool")
      .build()

    val byteColumnSchema = ZeusColumnSchema.newBuilder()
      .setFieldType(FieldType.BYTE)
      .setId(2)
      .setName("byte")
      .build()

    val floatColumnSchema = ZeusColumnSchema.newBuilder()
      .setFieldType(FieldType.FLOAT)
      .setId(3)
      .setName("float")
      .build()

    val intColumnSchema = ZeusColumnSchema.newBuilder()
      .setFieldType(FieldType.INT32)
      .setId(4)
      .setName("int")
      .build()

    val longColumnSchema = ZeusColumnSchema.newBuilder()
      .setFieldType(FieldType.INT64)
      .setId(5)
      .setName("long")
      .build()

    val stringColumnSchema = ZeusColumnSchema.newBuilder()
      .setFieldType(FieldType.STRING)
      .setId(6)
      .setName("string")
      .build()

    val tableSchema = ZeusTableSchema.newBuilder()
      .setFormat("simple")
      .setId(1)
      .setName("table")
      .putFields(1, boolColumnSchema)
      .putFields(2, byteColumnSchema)
      .putFields(3, floatColumnSchema)
      .putFields(4, intColumnSchema)
      .putFields(5, longColumnSchema)
      .putFields(6, stringColumnSchema)
      .build()

    val schemaOutput = new FileOutputStream("/home/liurenjie-sal/Downloads/test.schema")
    val codedOutput = CodedOutputStream.newInstance(schemaOutput)
    tableSchema.writeTo(codedOutput)
    codedOutput.flush()
    schemaOutput.flush()
    schemaOutput.close()


    val props = new Properties()
    props.put("output.type", "file")
    props.put("output.file", "/home/liurenjie-sal/Downloads/test.data")
    props.put("simple.block.row.num", "3")


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
