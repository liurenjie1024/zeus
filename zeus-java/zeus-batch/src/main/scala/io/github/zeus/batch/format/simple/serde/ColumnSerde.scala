package io.github.zeus.batch.format.simple.serde

import java.io.{InputStream, OutputStream}

import io.github.zeus.rpc.FieldType


trait ColumnSerde[T <: FieldType] {
  def serialize(column: Iterable[Any], output: OutputStream)
  def deserialize(input: InputStream): Iterable[Any]
}
