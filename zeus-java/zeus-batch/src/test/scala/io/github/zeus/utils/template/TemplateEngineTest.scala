package io.github.zeus.utils.template

import org.scalatest.{FunSuite, Matchers}

class TemplateEngineTest extends FunSuite with Matchers {
  test("sql should be transformed") {
    val sql = "select * from {{table}}"
    val template = new TemplateEngine

    val env = Map("table" -> "logs.parquet")

    val result =  template.transform(sql, env)
    result shouldBe "select * from logs.parquet"
  }

  test("Not found key should throw exception") {
    val sql = "select * from {{table}}"
    val template = new TemplateEngine

    assertThrows[NoSuchElementException] {
      template.transform(sql, Map())
    }
  }
}
