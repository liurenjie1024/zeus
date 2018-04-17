package io.github.zeus.batch.format.blizard.serde

import java.io.OutputStream
import java.nio.channels.Channels
import java.nio.{ByteBuffer, ByteOrder}

/**
  * Created by liurenjie on 24/03/2018.
  */
class ColumnOutputStream(private val outputStream: OutputStream) extends AutoCloseable {
  private val channel = Channels.newChannel(outputStream)
  private val buffer = ByteBuffer.allocate(8)
    .order(ByteOrder.LITTLE_ENDIAN)

  /**
    * Write bytes to target.
    * @param bytes Bytes to write.
    * @return Bytes written.
    */
  def write(bytes: Array[Byte]): Int = {
    channel.write(ByteBuffer.wrap(bytes))
    bytes.length
  }

  def write(value: Short): Int = {
    buffer.clear()
    buffer.putShort(value)
      .flip()
    channel.write(buffer)
    2
  }

  def write(value: Int): Int = {
    buffer.clear()
    buffer.putInt(value)
        .flip()
    channel.write(buffer)
    4
  }
  def write(value: Long): Int = {
    buffer.clear()
    buffer.putLong(value)
      .flip()
    channel.write(buffer)
    8
  }
  def write(value: Float): Int = {
    buffer.clear()
    buffer.putFloat(value)
      .flip()
    channel.write(buffer)
    4
  }
  def write(value: Double): Int = {
    buffer.clear()
    buffer.putDouble(value)
      .flip()
    channel.write(buffer)
    8
  }

  def write(value: String): Int = {
    val bytes = value.getBytes("utf-8")
    write(bytes)
  }

  def flush(): Unit = {}

  override def close(): Unit = channel.close()
}

object ColumnOutputStream {
  def main(args: Array[String]): Unit = {
    val buffer = ByteBuffer.allocate(8)
      .order(ByteOrder.LITTLE_ENDIAN)

    buffer.put("12".getBytes("utf-8"))
    println(buffer.array().map(Integer.toHexString(_)).mkString(","))
  }
}
