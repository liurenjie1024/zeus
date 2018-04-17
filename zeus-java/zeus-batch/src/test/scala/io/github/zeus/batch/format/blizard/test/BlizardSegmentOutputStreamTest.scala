package io.github.zeus.batch.format.blizard.test

import java.io.ByteArrayOutputStream
import java.nio.ByteBuffer
import java.util.Properties

import com.google.protobuf.CodedInputStream
import io.github.zeus.batch.format.blizard.BlizardSegmentOutputStream._
import io.github.zeus.batch.{Row, TableOutputStreamBuilder}
import io.github.zeus.format.simple.BlockHandles
import io.github.zeus.rpc.{ColumnType, ZeusColumnSchema, ZeusTableSchema}
import org.scalatest.{FunSuite, Matchers}

/**
  * Created by liurenjie on 25/03/2018.
  */
class BlizardSegmentOutputStreamTest extends FunSuite with Matchers {
  test("Output Content") {
    val boolColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(ColumnType.BOOL)
      .setId(1)
      .setName("bool")
      .build()

    val int8ColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(ColumnType.INT8)
      .setId(2)
      .setName("int8")
      .build()

    val int16ColumnSchema = ZeusColumnSchema.newBuilder()
      .setColumnType(ColumnType.INT8)
      .setId(2)
      .setName("int8")
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

    val props = new Properties()
    props.put("output.type", "memory")
    props.put("blizard.block.row.num", "2")


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
    tableOutput.close()

    val result = tableOutputBuilder.getOutput
      .asInstanceOf[ByteArrayOutputStream]
      .toByteArray

    val header = result.slice(0, 4)
    header shouldBe MagicNumber

    val version = result.slice(4, 8)
    version shouldBe Version

    val metaLength = ByteBuffer.wrap(result.slice(result.length-4, result.length)).getInt
    val blockHandles = BlockHandles.parseFrom(
      CodedInputStream.newInstance(result.slice(54, 54+metaLength)))

    blockHandles.getMaxBlockColumnSize shouldBe 2
    blockHandles.getHandlesCount should === (1)

    val blockHandle = blockHandles.getHandles(0)
    blockHandle.getStart should === (8)
    blockHandle.getEnd should === (54)
    blockHandle.getBlockColumnSize should === (2)

    val boolColumn = result.slice(8, 9)
    boolColumn shouldBe Array(0x01).map(_.toByte)
    val boolColumnHandle = blockHandle.getColumnsOrThrow(1)
    boolColumnHandle.getStart shouldBe 8
    boolColumnHandle.getEnd shouldBe 9

    val byteColumn = result.slice(9, 11)
    byteColumn shouldBe Array(0x01, 0x02).map(_.toByte)
    val byteColumnHandle = blockHandle.getColumnsOrThrow(2)
    byteColumnHandle.getStart shouldBe 9
    byteColumnHandle.getEnd shouldBe 11

    val floatColumn = result.slice(11, 19)
    floatColumn shouldBe Array(0x3F, 0x80, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00).map(_.toByte)
    val floatColumnHandle = blockHandle.getColumnsOrThrow(3)
    floatColumnHandle.getStart shouldBe 11
    floatColumnHandle.getEnd shouldBe 19

    val intColumn = result.slice(19, 27)
    intColumn shouldBe Array(0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02).map(_.toByte)
    val intColumnHandle = blockHandle.getColumnsOrThrow(4)
    intColumnHandle.getStart shouldBe 19
    intColumnHandle.getEnd shouldBe 27

    val longColumn = result.slice(27, 43)
    longColumn shouldBe Array(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02).map(_.toByte)
    val longColumnHandle = blockHandle.getColumnsOrThrow(5)
    longColumnHandle.getStart shouldBe 27
    longColumnHandle.getEnd shouldBe 43

    val stringColumn = result.slice(43, 54)
    stringColumn shouldBe Array(0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x31,
      0x31, 0x32).map(_.toByte)
    val stringColumnHandle = blockHandle.getColumnsOrThrow(6)
    stringColumnHandle.getStart shouldBe 43
    stringColumnHandle.getEnd shouldBe 54
  }
}
