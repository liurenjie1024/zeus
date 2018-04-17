package io.github.zeus.batch.format.blizard

import java.io.{ByteArrayOutputStream, OutputStream}
import java.util.Properties

import com.google.protobuf.CodedOutputStream
import io.github.zeus.batch.format.blizard.BlizardSegmentOutputStream._
import io.github.zeus.batch.format.blizard.FieldHelper._
import io.github.zeus.batch.format.blizard.serde.ColumnOutputStream
import io.github.zeus.batch.{Row, TableOutputStream, TableOutputStreamBuilder}
import io.github.zeus.format.blizard.{BlockNode, ColumnNode, SegmentIndex}

import scala.collection.JavaConverters._

class BlizardSegmentOutputStream(config: Properties,
                                 indexOutput: OutputStream,
                                 dataOutput: OutputStream) extends TableOutputStream {
  private val destination: ColumnOutputStream = new ColumnOutputStream(builder.getOutput)


  private val blockRowNum = builder.config.getProperty("blizard.block.row.num").toInt
  require(blockRowNum > 0)

  private val block: Array[Row] = Array.fill(blockRowNum)(null)
  private var nextIndex: Int = 0
  private val segmentIndexBuilder = SegmentIndex.newBuilder()

  override def write(row: Row): Unit = {
    require(row != null, "row can't be null!")
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

    val blockStart = segmentIndexBuilder.getBlockNodeBuilderList.asScala.lastOption
      .map(_.getEnd)
      .getOrElse(0)

    val blockNodeBuilder = BlockNode.newBuilder()
      .setStart(blockStart)

    var columnStart = blockStart
    for (columnId <- builder.tableSchema.getColumnsMap.keySet().asScala) {
      val columnSchema = builder.tableSchema.getColumnsOrThrow(columnId)
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

object BlizardSegmentOutputStream {
  private[blizard] val MagicNumber = Array(0xAA, 0xBB, 0xCC, 0xDD).map(_.toByte)
  private[blizard] val Version = Array(0x00, 0x00, 0x00, 0x01).map(_.toByte)
}
