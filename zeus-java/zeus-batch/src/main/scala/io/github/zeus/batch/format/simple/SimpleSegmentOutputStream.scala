package io.github.zeus.batch.format.simple

import com.google.protobuf.CodedOutputStream
import io.github.zeus.batch.format.simple.FieldHelper._
import io.github.zeus.batch.format.simple.SimpleSegmentOutputStream._
import io.github.zeus.batch.format.simple.serde.ColumnOutputStream
import io.github.zeus.batch.{Row, TableOutputStream, TableOutputStreamBuilder}
import io.github.zeus.format.simple.{BlockHandle, BlockHandles, ColumnHandle}

import scala.math.max
import scala.collection.JavaConverters._


class SimpleSegmentOutputStream(builder: TableOutputStreamBuilder) extends TableOutputStream {
  private val destination: ColumnOutputStream = new ColumnOutputStream(builder.getOutput)


  private val blockRowNum = builder.config.getProperty("simple.block.row.num").toInt
  require(blockRowNum > 0)

  private val block: Array[Row] = Array.fill(blockRowNum)(null)
  private var nextIndex: Int = 0
  private val blockHandlesBuilder = BlockHandles.newBuilder()

  private var headerWritten: Boolean = false

  override def write(row: Row): Unit = {
    if (!isHeaderWritten) {
      writeHeader
    }

    block(nextIndex) = row
    nextIndex += 1

    if (isBlockFull) {
      writeBlockAndClear
    }
  }

  override def close(): Unit = {
    writeBlockAndClear
    writeBlockIndexes
    destination.flush()
    destination.close()
  }

  private def writeBlockAndClear: Unit = {
    val blockStart = blockHandlesBuilder.getHandlesList.asScala.lastOption
      .map(_.getEnd)
      .getOrElse((MagicNumber.length+VERSION.length).toLong)

    val blockHandleBuilder = BlockHandle.newBuilder()
      .setStart(blockStart)

    val nonEmptyBlocks = block.take(nextIndex)
    var columnStart = blockStart
    for (columnSchema <- builder.tableSchema.getFieldsMap.values().asScala) {
      val columnId = columnSchema.getId
      val column = nonEmptyBlocks
        .map(_.getColumnValue(columnId).get)
        .iterator

      val bytesWritten = columnSchema.getFieldType
        .serialize(column, destination)

      val columnHandle = ColumnHandle.newBuilder()
        .setStart(columnStart)
        .setEnd(columnStart + bytesWritten)
        .build()
      blockHandleBuilder.putColumns(columnId, columnHandle)
      columnStart += bytesWritten
    }

    val blockHandle =  blockHandleBuilder.setEnd(columnStart)
      .setBlockColumnSize(nonEmptyBlocks.length)
      .build()

    blockHandlesBuilder.addHandles(blockHandle)
      .setMaxBlockColumnSize(max(blockHandlesBuilder.getMaxBlockColumnSize, nonEmptyBlocks.length))

    block.indices.foreach(idx => block(idx) = null)
    nextIndex = 0
  }

  private def isBlockFull: Boolean = {
    nextIndex >= block.length
  }

  private def writeBlockIndexes: Unit = {
    val output = CodedOutputStream.newInstance(destination.getOutput)
    blockHandlesBuilder.build()
      .writeTo(output)
    destination.write(output.getTotalBytesWritten)
  }

  private def writeHeader: Unit = {
    destination.write(MagicNumber)
    destination.write(VERSION)

    headerWritten = true
  }
  private def isHeaderWritten: Boolean = headerWritten
}

object SimpleSegmentOutputStream {
  private val MagicNumber = Array(0xAA, 0xBB, 0xCC, 0xDD).map(_.toByte)
  private val VERSION = Array(0x00, 0x00, 0x00, 0x01).map(_.toByte)
}
