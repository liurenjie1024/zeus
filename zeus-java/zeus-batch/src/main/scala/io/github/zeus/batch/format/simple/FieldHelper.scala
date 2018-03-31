package io.github.zeus.batch.format.simple

import io.github.zeus.batch.format.simple.serde._
import io.github.zeus.rpc.ColumnType
import io.github.zeus.rpc.ColumnType._


/**
  * Created by liurenjie on 24/03/2018.
  */
object FieldHelper {
  implicit class FieldImprovement(private val filedType: ColumnType) {
    def serialize(values: Iterator[Any], output: ColumnOutputStream): Int = {
      filedType match {
        case BOOL => BooleanColumnSerde.serialize(values.map(_.asInstanceOf[Boolean]), output)
        case BYTE => ByteColumnSerde.serialize(values.map(_.asInstanceOf[Byte]), output)
        case FLOAT => FloatColumnSerde.serialize(values.map(_.asInstanceOf[Float]), output)
        case INT32 => IntColumnSerde.serialize(values.map(_.asInstanceOf[Int]), output)
        case INT64 => LongColumnSerde.serialize(values.map(_.asInstanceOf[Long]), output)
        case STRING => StringColumnSerde.serialize(values.map(_.asInstanceOf[String]), output)
        case TIMESTAMP => LongColumnSerde.serialize(values.map(_.asInstanceOf[Long]), output)
        case t => throw new IllegalArgumentException(s"Unrecognized field type: ${t.name()}")
      }
    }
    
  }
}
