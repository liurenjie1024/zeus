import java.io.{FileOutputStream, FileReader}

import com.google.protobuf.util.JsonFormat
import com.google.protobuf.{CodedOutputStream, Message}
import io.github.zeus.common.AutoManageResources._
import org.rogach.scallop._

/**
  * Parse json protobuf and write it to stdout using binary encoding.
  */
object ProtobufJsonParser {
  class Conf(arguments: Seq[String]) extends ScallopConf(arguments) {
    val input = opt[String]("input", 'i', required = true)
    val output = opt[String]("output", 'o', required = true)
    val className = opt[String]("className", 'c', required = true)
    verify()
  }

  def main(args: Array[String]): Unit = {
    val conf = new Conf(args)

    new FileReader(conf.input()).flatMap { reader =>
      val messageClass = Class.forName(conf.className())
      val messageBuilder = messageClass.getMethod("newBuilder").invoke(null)
        .asInstanceOf[Message.Builder]

      JsonFormat.parser().merge(reader, messageBuilder)

      new FileOutputStream(conf.output()).flatMap { fout =>
        val output = CodedOutputStream.newInstance(fout)
        messageBuilder.build().writeTo(output)
        output.flush()
      }
    }
  }
}
