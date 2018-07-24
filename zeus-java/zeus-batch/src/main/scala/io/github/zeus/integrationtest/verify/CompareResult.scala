package io.github.zeus.integrationtest.verify

import java.io.PrintWriter

import io.github.zeus.integrationtest.SqlElement

case class CompareResult(sql: SqlElement, same: Boolean,
  expectedRS: Option[JDBCResultSet], actualRS: Option[JDBCResultSet], failure: Option[Throwable])

case class CompareResults(sameCount: Int, differentCount: Int, failureCount: Int,
  results: List[CompareResult]) {
  def output(writer: PrintWriter): Unit = {
    writer.println(s"""Same Count: ${sameCount}
                      |different count: ${differentCount}
                      |failure count: ${failureCount}""".stripMargin
      .replaceAll("\n", ","))

    writer.println()

    for (r <- results) {
      writer.println()
      writer.println(s"sql: ${r.sql}, same: ${r.same}")
      writer.println("========================Expected results:========================")
      r.expectedRS.foreach(_.output(writer))
      writer.println("========================Actual results:========================")
      r.actualRS.foreach(_.output(writer))
      writer.println("========================Failure:========================")
      r.failure.foreach(_.printStackTrace(writer))
    }

    writer.flush()
  }
}
