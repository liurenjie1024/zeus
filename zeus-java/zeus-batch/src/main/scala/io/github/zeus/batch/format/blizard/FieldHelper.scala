package io.github.zeus.batch.format.blizard

import io.github.zeus.batch.format.blizard.serde._
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
        case INT8 => ByteColumnSerde.serialize(values.map(_.asInstanceOf[Byte]), output)
        case INT16=> Int16ColumnSerde.serialize(values.map(_.asInstanceOf[Short]), output)
        case INT32 => IntColumnSerde.serialize(values.map(_.asInstanceOf[Int]), output)
        case INT64 => LongColumnSerde.serialize(values.map(_.asInstanceOf[Long]), output)
        case FLOAT4 => FloatColumnSerde.serialize(values.map(_.asInstanceOf[Float]), output)
        case FLOAT8 => DoubleColumnSerde.serialize(values.map(_.asInstanceOf[Double]), output)
        case STRING => StringColumnSerde.serialize(values.map(_.asInstanceOf[String]), output)
        case TIMESTAMP => LongColumnSerde.serialize(values.map(_.asInstanceOf[Long]), output)
        case t => throw new IllegalArgumentException(s"Unrecognized field type: ${t.name()}")
      }
    }
    
  }
}
