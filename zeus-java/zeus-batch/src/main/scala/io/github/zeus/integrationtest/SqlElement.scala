package io.github.zeus.integrationtest

import java.io.{FileInputStream, InputStream}

import scala.xml.{Elem, XML}

case class SqlElement(name: String, isOrdered: Boolean, sql: String)

object SqlElement {
  def from(xml: Elem): SqlElement = {
    new SqlElement(name = getAttribute(xml, "name").get,
      isOrdered = getAttribute(xml, "isOrdered").getOrElse("false").toBoolean,
      sql = xml.text.trim)
  }

  def getAttribute(xml: Elem, key: String): Option[String] = {
    xml.attribute(key).map(x => x(0).text)
  }

  def loadFile(filename: String): Seq[SqlElement] = {
    val input = new FileInputStream(filename)
    try {
      load(input)
    } finally {
      input.close()
    }
  }

  def load(input: InputStream): Seq[SqlElement] = {
    val sqls = (XML.load(input) \ "sql")
      .map(x => SqlElement.from(x.asInstanceOf[Elem]))

    val duplicateNames = sqls.groupBy(_.name)
      .collect({
        case (name, ys) if ys.size > 1 => name
      })

    if (duplicateNames.size > 0) {
      throw new IllegalArgumentException(s"${duplicateNames} is duplicated.")
    }

    sqls
  }
}
