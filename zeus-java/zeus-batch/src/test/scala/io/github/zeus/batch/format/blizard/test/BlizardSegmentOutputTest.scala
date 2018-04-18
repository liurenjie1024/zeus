package io.github.zeus.batch.format.blizard.test

import java.io.ByteArrayOutputStream
import java.util.Properties

import io.github.zeus.batch.Row
import io.github.zeus.batch.format.blizard.BlizardSegmentOutput
import io.github.zeus.format.blizard.SegmentIndex
import io.github.zeus.rpc.ColumnType._
import io.github.zeus.rpc.{ZeusColumnSchema, ZeusTableSchema}
import org.scalatest.{FunSuite, Matchers}

/**
  * Created by liurenjie on 25/03/2018.
  */
class BlizardSegmentOutputTest extends FunSuite with Matchers {
  test("Output Content") {
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
      .setName("table")
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

    val props = new Properties()
    props.put("blizard.block.row.num", "2")


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


    val indexOutput = new ByteArrayOutputStream()
    val dataOutput = new ByteArrayOutputStream()
    val tableOutput = new BlizardSegmentOutput(props, tableSchema, indexOutput, dataOutput)
    tableOutput.write(new Row(row1))
    tableOutput.write(new Row(row2))
    tableOutput.close()

    val segmentIndex = SegmentIndex.parseFrom(indexOutput.toByteArray)

    segmentIndex.getBlockNodeCount shouldBe 1

    val blockNode = segmentIndex.getBlockNode(0)
    blockNode.getColumnNodeCount shouldBe 9
    blockNode.getBlockColumnSize shouldBe 2
    blockNode.getStart shouldBe 0
    blockNode.getEnd shouldBe 86 //TODO: Fix

    val result = dataOutput.toByteArray

    val boolColumn = result.slice(0, 2)
    boolColumn shouldBe Array(0x01, 0x00).map(_.toByte)
    val boolColumnHandle = blockNode.getColumnNodeOrThrow(1)
    boolColumnHandle.getStart shouldBe 0
    boolColumnHandle.getEnd shouldBe 2

    val int8Column = result.slice(2, 4)
    int8Column shouldBe Array(0x01, 0x02).map(_.toByte)
    val int8ColumnHandle = blockNode.getColumnNodeOrThrow(2)
    int8ColumnHandle.getStart shouldBe 2
    int8ColumnHandle.getEnd shouldBe 4

    val int16Column = result.slice(4, 8)
    int16Column shouldBe Array(0x01, 0x00, 0x02, 0x00).map(_.toByte)
    val int16ColumnHandle = blockNode.getColumnNodeOrThrow(3)
    int16ColumnHandle.getStart shouldBe 4
    int16ColumnHandle.getEnd shouldBe 8

    val int32Column = result.slice(8, 16)
    int32Column shouldBe Array(0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00).map(_.toByte)
    val int32ColumnHandle = blockNode.getColumnNodeOrThrow(4)
    int32ColumnHandle.getStart shouldBe 8
    int32ColumnHandle.getEnd shouldBe 16

    val int64Column = result.slice(16, 32)
    int64Column shouldBe Array(0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00).map(_.toByte)
    val int64ColumnHandle = blockNode.getColumnNodeOrThrow(5)
    int64ColumnHandle.getStart shouldBe 16
    int64ColumnHandle.getEnd shouldBe 32

    val float4Column = result.slice(32, 40)
    float4Column shouldBe Array(0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x00, 0x40).map(_.toByte)
    val float4ColumnHandle = blockNode.getColumnNodeOrThrow(6)
    float4ColumnHandle.getStart shouldBe 32
    float4ColumnHandle.getEnd shouldBe 40

    val float8Column = result.slice(40, 56)
    float8Column shouldBe Array(0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40)
      .map(_.toByte)
    val float8ColumnHandle = blockNode.getColumnNodeOrThrow(7)
    float8ColumnHandle.getStart shouldBe 40
    float8ColumnHandle.getEnd shouldBe 56

    val timestampColumn = result.slice(56, 72)
    timestampColumn shouldBe Array(0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00).map(_.toByte)
    val timestampColumnHandle = blockNode.getColumnNodeOrThrow(8)
    timestampColumnHandle .getStart shouldBe 56
    timestampColumnHandle .getEnd shouldBe 72

    val stringColumn = result.slice(72, 86)
    stringColumn shouldBe Array(0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
      0x00, 0x31, 0x32).map(_.toByte)
    val stringColumnHandle = blockNode.getColumnNodeOrThrow(9)
    stringColumnHandle.getStart shouldBe 72
    stringColumnHandle.getEnd shouldBe 86
  }
}
