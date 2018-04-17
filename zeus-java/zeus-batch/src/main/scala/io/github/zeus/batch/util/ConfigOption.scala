package io.github.zeus.batch.util

import java.util.Properties

case class ConfigOption[T](key: String) {
  def get(props: Properties)(implicit toValue: String => T): Option[T] = {
    Option(props.getProperty(key))
      .map(toValue)
  }
}

object ConfigOption {
  implicit val ToStringValue: String => String = (v: String) => v
  implicit val ToIntValue: String => Int = (v: String) => Integer.parseInt(v)
}


