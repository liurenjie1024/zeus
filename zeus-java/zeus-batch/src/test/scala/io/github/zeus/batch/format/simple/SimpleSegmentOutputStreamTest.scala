package io.github.zeus.batch.format.simple

import java.io.ByteArrayOutputStream
import java.util.Properties

import io.github.zeus.batch.{Row, TableOutputStreamBuilder}
import io.github.zeus.rpc.{FieldType, ZeusColumnSchema, ZeusTableSchema}
import org.scalatest.{FunSuite, Matchers}

/**
  * Created by liurenjie on 25/03/2018.
  */
class SimpleSegmentOutputStreamTest extends FunSuite with Matchers {
  test("Output Content") {
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

    val props = new Properties()
    props.put("output.type", "memory")
    props.put("simple.block.row.num", "2")


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

    val tableOutputBuilder = TableOutputStreamBuilder(tableSchema, props)

    val tableOutput = tableOutputBuilder.build
    tableOutput.write(new Row(row1))
    tableOutput.write(new Row(row2))

    val result = tableOutputBuilder.getOutput
      .asInstanceOf[ByteArrayOutputStream]
      .toByteArray

    val header = result.slice(0, 4)
    header shouldBe Array(0xAA, 0xBB, 0xCC, 0xDD).map(_.toByte)

    val version = result.slice(4, 8)
    version shouldBe Array(0x01, 0x00, 0x00, 0x01).map(_.toByte)
  }
}
