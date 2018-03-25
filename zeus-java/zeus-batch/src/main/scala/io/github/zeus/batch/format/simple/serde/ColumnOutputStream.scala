package io.github.zeus.batch.format.simple.serde

import java.io.{DataOutputStream, OutputStream}

/**
  * Created by liurenjie on 24/03/2018.
  */
class ColumnOutputStream(private val output: OutputStream) extends AutoCloseable {
  private val dataOutput = new DataOutputStream(output)
  /**
    * Write bytes to target.
    * @param bytes Bytes to write.
    * @return Bytes written.
    */
  def write(bytes: Array[Byte]): Int = {
    dataOutput.write(bytes)
    bytes.length
  }
  def write(value: Int): Int = {
    dataOutput.writeInt(value)
    4
  }
  def write(value: Long): Int = {
    dataOutput.writeLong(value)
    8
  }
  def write(value: Float): Int = {
    dataOutput.writeFloat(value)
    4
  }

  def write(value: String): Int = {
    val bytes = value.getBytes("utf-8")
    write(bytes)
  }

  def flush(): Unit = dataOutput.flush()

  override def close(): Unit = dataOutput.close()

  def getOutput: OutputStream = output
}
