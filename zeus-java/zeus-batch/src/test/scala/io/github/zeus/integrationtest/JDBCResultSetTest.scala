package io.github.zeus.integrationtest

import io.github.zeus.integrationtest.verify.JDBCResultSet
import org.scalatest.{FunSuite, Matchers}

class JDBCResultSetTest extends FunSuite with Matchers {
  test("Equal unordered result set") {
    val rs1 = JDBCResultSet(List("advId", "expr1"),
      List(1, 2), List("int", "string"),
      List(List[AnyRef](Integer.valueOf(1), "bb"),
        List[AnyRef](Integer.valueOf(2), "aa"),
        List[AnyRef](Integer.valueOf(1), "bb")))

    val rs2 = JDBCResultSet(List("advId", "expr1"),
      List(1, 2), List("int", "string"),
      List(List[AnyRef](Integer.valueOf(2), "aa"),
        List[AnyRef](Integer.valueOf(1), "bb"),
        List[AnyRef](Integer.valueOf(1), "bb")))

    JDBCResultSet.compareUnordered(rs1, rs2) shouldBe true
    JDBCResultSet.compareOrdered(rs1, rs2) shouldBe false
  }

  test("Equal ordered result set") {
    val rs1 = JDBCResultSet(List("advId", "expr1"),
      List(1, 2), List("int", "string"),
      List(List[AnyRef](Integer.valueOf(1), "bb"),
        List[AnyRef](Integer.valueOf(2), "aa"),
        List[AnyRef](Integer.valueOf(1), "bb")))

    val rs2 = JDBCResultSet(List("advId", "expr1"),
      List(1, 2), List("int", "string"),
      List(List[AnyRef](Integer.valueOf(1), "bb"),
        List[AnyRef](Integer.valueOf(2), "aa"),
        List[AnyRef](Integer.valueOf(1), "bb")))

    JDBCResultSet.compareUnordered(rs1, rs2) shouldBe true
    JDBCResultSet.compareOrdered(rs1, rs2) shouldBe true
  }

  test("Unequal metadata") {
    val rs1 = JDBCResultSet(List("advId", "expr1"),
      List(1, 2), List("int", "string"),
      List())

    val rs2 = JDBCResultSet(List("advId", "expr1"),
      List(1, 1), List("int", "int"),
      List())

    JDBCResultSet.compareUnordered(rs1, rs2) shouldBe false
    JDBCResultSet.compareOrdered(rs1, rs2) shouldBe false
  }

  test("Unequal data") {
    val rs1 = JDBCResultSet(List("advId", "expr1"),
      List(1, 2), List("int", "string"),
      List(List[AnyRef](Integer.valueOf(1), "bb"),
        List[AnyRef](Integer.valueOf(2), "aa"),
        List[AnyRef](Integer.valueOf(1), "bb")))

    val rs2 = JDBCResultSet(List("advId", "expr1"),
      List(1, 2), List("int", "string"),
      List(List[AnyRef](Integer.valueOf(2), "aa"),
        List[AnyRef](Integer.valueOf(1), "bb")))

    JDBCResultSet.compareUnordered(rs1, rs2) shouldBe false
    JDBCResultSet.compareOrdered(rs1, rs2) shouldBe false
  }
}
