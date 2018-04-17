package io.github.zeus.batch.format.blizard.serde

import java.io.ByteArrayOutputStream
import java.nio.channels.Channels
import java.util


trait ColumnSerde[A] {
  /**
    * Serialize column to output stream.
    * @param column
    * @param output
    * @return Bytes written.
    */
  def serialize(column: Iterator[A], output: ColumnOutputStream): Int
}

object BooleanColumnSerde extends ColumnSerde[Boolean] {
  /**
    * Serialize column to output stream.
    *
    * @param column
    * @param output
    * @return Bytes written.
    */
  override def serialize(column: Iterator[Boolean], output: ColumnOutputStream): Int = {
    output.write(column.map(x => if(x) 1 else 0).map(_.toByte).toArray)
  }
}

object ByteColumnSerde extends ColumnSerde[Byte] {
  /**
    * Serialize column to output stream.
    *
    * @param column
    * @param output
    * @return Bytes written.
    */
  override def serialize(column: Iterator[Byte], output: ColumnOutputStream): Int = {
    output.write(column.toArray)
  }
}

object Int16ColumnSerde extends ColumnSerde[Short] {
  /**
    * Serialize column to output stream.
    *
    * @param column
    * @param output
    * @return Bytes written.
    */
  override def serialize(column: Iterator[Short], output: ColumnOutputStream): Int = {
    column.map(output.write)
      .sum
  }
}

object FloatColumnSerde extends ColumnSerde[Float] {
  /**
    * Serialize column to output stream.
    *
    * @param column
    * @param output
    * @return Bytes written.
    */
  override def serialize(column: Iterator[Float], output: ColumnOutputStream): Int = {
    column.map(output.write)
      .sum
  }
}

object DoubleColumnSerde extends ColumnSerde[Double] {
  /**
    * Serialize column to output stream.
    *
    * @param column
    * @param output
    * @return Bytes written.
    */
  override def serialize(column: Iterator[Double], output: ColumnOutputStream): Int = {
    column.map(output.write)
      .sum
  }
}

object IntColumnSerde extends ColumnSerde[Int] {
  /**
    * Serialize column to output stream.
    *
    * @param column
    * @param output
    * @return Bytes written.
    */
  override def serialize(
    column: Iterator[Int],
    output: ColumnOutputStream): Int = {
    column.map(output.write)
      .sum
  }
}

object LongColumnSerde extends ColumnSerde[Long] {
  /**
    * Serialize column to output stream.
    *
    * @param column
    * @param output
    * @return Bytes written.
    */
  override def serialize(column: Iterator[Long], output: ColumnOutputStream): Int = {
    column.map(output.write)
      .sum
  }
}

object StringColumnSerde extends ColumnSerde[String] {
  /**
    * Serialize column to output stream.
    *
    * @param column
    * @param output
    * @return Bytes written.
    */
  override def serialize(column: Iterator[String], output: ColumnOutputStream): Int = {
    val buffer = new ByteArrayOutputStream()
    val bufferOutput = new ColumnOutputStream(Channels.newChannel(buffer))

    var bytesWritten = 0
    var pos = 0
    bytesWritten += output.write(0)
    for (s <- column) {
      pos += bufferOutput.write(s)
      bytesWritten += output.write(pos)
    }
    bufferOutput.flush()
    bufferOutput.close()

    bytesWritten += output.write(buffer.toByteArray)
    bytesWritten
  }
}

