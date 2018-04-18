package io.github.zeus.batch.format.blizard

import java.io.OutputStream
import java.util.Properties

import com.google.protobuf.CodedOutputStream
import io.github.zeus.batch.format.blizard.BlizardSegmentOutput._
import io.github.zeus.batch.format.blizard.FieldHelper._
import io.github.zeus.batch.format.blizard.serde.ColumnOutputStream
import io.github.zeus.batch.util.ConfigOption
import io.github.zeus.batch.{Row, TableOutput}
import io.github.zeus.format.blizard.{BlockNode, ColumnNode, SegmentIndex}
import io.github.zeus.rpc.ZeusTableSchema

import ConfigOption._
import scala.collection.JavaConverters._

class BlizardSegmentOutput(config: Properties,
  tableSchema: ZeusTableSchema,
  indexOutput: OutputStream,
  dataOutput: OutputStream) extends TableOutput {
  private val destination: ColumnOutputStream = new ColumnOutputStream(dataOutput)

  private val blockRowNum = ConfigMaxBlockRowNum.get(config).get
  require(blockRowNum > 0)

  private val block: Array[Row] = Array.fill(blockRowNum)(null)
  private var nextIndex: Int = 0
  private val segmentIndexBuilder = SegmentIndex.newBuilder()

  override def write(row: Row): Unit = {
    require(row != null, "row can't be null!")

    block(nextIndex) = row
    nextIndex += 1

    if (isBlockFull) {
      writeBlockAndClear()
    }
  }

  override def close(): Unit = {
    writeBlockAndClear()
    writeBlockIndexes()
    destination.flush()
    destination.close()
  }

  private def writeBlockAndClear(): Unit = {
    val nonEmptyBlocks = block.take(nextIndex)
    if (nonEmptyBlocks.isEmpty) {
      return
    }

    val blockStart = segmentIndexBuilder.getBlockNodeBuilderList.asScala.lastOption
      .map(_.getEnd)
      .getOrElse(0L)

    val blockNodeBuilder = BlockNode.newBuilder()
      .setStart(blockStart)

    var columnStart = blockStart
    for (columnId <- tableSchema.getColumnsMap.keySet().asScala.toSeq.sorted) {
      val columnSchema = tableSchema.getColumnsOrThrow(columnId)
      val column = nonEmptyBlocks
        .map(_.getColumnValue(columnId).get)
        .iterator

      val bytesWritten = columnSchema.getColumnType
        .serialize(column, destination)

      val columnNode = ColumnNode.newBuilder()
        .setStart(columnStart)
        .setEnd(columnStart + bytesWritten)
        .build()
      blockNodeBuilder.putColumnNode(columnId, columnNode)
      columnStart += bytesWritten
    }

    val blockNode =  blockNodeBuilder.setEnd(columnStart)
      .setBlockColumnSize(nonEmptyBlocks.length)
      .build()

    segmentIndexBuilder.addBlockNode(blockNode)

    block.indices.foreach(idx => block(idx) = null)
    nextIndex = 0
  }

  private def isBlockFull: Boolean = {
    nextIndex >= block.length
  }

  private def writeBlockIndexes(): Unit = {
    val output = CodedOutputStream.newInstance(indexOutput)
    segmentIndexBuilder.build()
        .writeTo(output)
    output.flush()
    indexOutput.flush()
    indexOutput.close()
  }
}

object BlizardSegmentOutput {
  val ConfigMaxBlockRowNum: ConfigOption[Int] = ConfigOption[Int]("blizard.block.row.num")
}
