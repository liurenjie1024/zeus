package io.github.zeus.batch.format.simple

import java.io.OutputStream

import io.github.zeus.batch.{Row, TableOutputStream, TableOutputStreamBuilder}
import io.github.zeus.format.simple.{BlockHandle, BlockHandles}

import scala.collection.JavaConversions._
import SimpleSegmentOutputStream._

class SimpleSegmentOutputStream(builder: TableOutputStreamBuilder) extends TableOutputStream {
  private val destination: OutputStream = builder.getOutputStream

  private val blockRowNum = builder.config.getProperty("simple.block.row.num").toInt
  require(blockRowNum > 0)

  private val block: Array[Row] = Array.fill(blockRowNum)(null)
  private var nextIndex: Int = 0
  private val blockHandles = new BlockHandles.Builder()
    .setBlockColumnSize(blockRowNum)
    .build()

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
    val blockStart = blockHandles.getHandlesList.lastOption
      .map(_.getEnd)
      .getOrElse(MagicNumer.length+VERSION.length)

    val blockHandle = new BlockHandle.Builder()
      .setStart(blockStart)
      .build()

    var nextColumnStart = blockStart
    for (columnSchema <- builder.tableSchema.getFieldsMap.values()) {
      block.map(_.getColumnValue(columnSchema.getId))

    }
  }

  private def isBlockFull: Boolean = {
    nextIndex >= block.length
  }

  private def writeBlockIndexes: Unit = ???

  private def writeHeader: Unit = {
    destination.write(MagicNumer)
    destination.write(VERSION)

    headerWritten = true
  }
  private def isHeaderWritten: Boolean = headerWritten
}

object SimpleSegmentOutputStream {
  private val MagicNumer = Array(0xAA, 0xBB, 0xCC, 0xDD).map(_.toByte)
  private val VERSION = Array(0x00, 0x00, 0x00, 0x01).map(_.toByte)
}
