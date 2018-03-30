package io.github.zeus.batch.format.simple

import java.io.ByteArrayOutputStream

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
    val nonEmptyBlocks = block.take(nextIndex)
    if (nonEmptyBlocks.isEmpty) {
      return
    }

    val blockStart = blockHandlesBuilder.getHandlesList.asScala.lastOption
      .map(_.getEnd)
      .getOrElse((MagicNumber.length+Version.length).toLong)

    val blockHandleBuilder = BlockHandle.newBuilder()
      .setStart(blockStart)

    var columnStart = blockStart
    for (columnId <- builder.tableSchema.getFieldsMap.keySet().asScala) {
      val columnSchema = builder.tableSchema.getFieldsOrThrow(columnId)
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
    val buffer = new ByteArrayOutputStream()
    val output = CodedOutputStream.newInstance(buffer)
    blockHandlesBuilder.build()
      .writeTo(output)
    output.flush()
    val indexBytes = buffer.toByteArray
    println(s"Index length: ${indexBytes.length}")
    println(s"Index bytes: ${indexBytes.mkString("[", ",", "]")}")
    destination.write(indexBytes)
    destination.write(indexBytes.length)
  }

  private def writeHeader: Unit = {
    destination.write(MagicNumber)
    destination.write(Version)

    headerWritten = true
  }
  private def isHeaderWritten: Boolean = headerWritten
}

object SimpleSegmentOutputStream {
  private[simple] val MagicNumber = Array(0xAA, 0xBB, 0xCC, 0xDD).map(_.toByte)
  private[simple] val Version = Array(0x00, 0x00, 0x00, 0x01).map(_.toByte)
}
