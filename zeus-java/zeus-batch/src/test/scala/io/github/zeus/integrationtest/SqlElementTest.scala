package io.github.zeus.integrationtest

import org.scalatest.{FunSuite, Matchers}

class SqlElementTest extends FunSuite with Matchers {
  test("Load sql element") {
    val sqlElements = SqlElement.load(getClass.getClassLoader.getResourceAsStream("sqls.xml"))

    val expectedSqlElements = List(
      SqlElement("q1", true, "select * from logs"),
      SqlElement("q2", false, "select count(1) from logs")
    )

    sqlElements shouldBe expectedSqlElements
  }

  test("Sql with duplicate names should throw exception") {
    assertThrows[IllegalArgumentException] {
     SqlElement.load(getClass.getClassLoader.getResourceAsStream("duplicate_sqls.xml"))
    }
  }
}
