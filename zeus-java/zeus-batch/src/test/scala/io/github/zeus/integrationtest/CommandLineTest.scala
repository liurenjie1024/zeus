package io.github.zeus.integrationtest

import org.scalatest.{FunSuite, Matchers}

class CommandLineTest extends FunSuite with Matchers {
  test("Subcommand") {
    val args = "verify --jdbcUrl drill --jdbcDriver org.apache.drill.jdbc.Driver --sqlFilename src/main/resources/sqls.xml --drillTablename xx --zeusTablename xx --outputFilename results".split(" ")
//    args.foreach(println)
    val commandLine = new CommandLine(args)

//    println(commandLine.summary)
  }
}
