package io.github.zeus.batch.format.simple

implicit import io.github.zeus.batch.format.simple.serde.{BooleanColumnSerde, ByteColumnSerde,
 ColumnOutputStream, ColumnSerde}
import io.github.zeus.rpc.FieldType
import io.github.zeus.rpc.FieldType._


/**
  * Created by liurenjie on 24/03/2018.
  */
object FieldHelper {
  implicit class FieldImprovement(private val filedType: FieldType) {
    def serialize(values: Iterator[Any], output: ColumnOutputStream): Int = {
      filedType match {
        case BOOL => BooleanColumnSerde.serialize(values.map(_.asInstanceOf[Byte]), output)
        case BYTE => BooleanColumnSerde.serialize(va)
      }
    }
    
  }
}
