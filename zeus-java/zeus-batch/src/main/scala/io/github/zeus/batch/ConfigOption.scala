package io.github.zeus.batch

import java.io.{ByteArrayOutputStream, FileOutputStream, OutputStream}
import java.util.Properties

trait ConfigOption[T] {
  def get(implicit props: Properties): T
}

object OutputConfigOption extends ConfigOption[OutputStream] {
  override def get(implicit props: Properties): OutputStream = {
    val outputType = props.getProperty("output.type", "memory")

    outputType match {
      case "memory" => new ByteArrayOutputStream()
      case "file" => {
        val outputFile = props.getProperty("output.file")
        require(outputFile != null)
        new FileOutputStream(outputFile)
      }
      case s => throw new IllegalArgumentException(s"Unknow output type: ${s}")
    }
  }
}


