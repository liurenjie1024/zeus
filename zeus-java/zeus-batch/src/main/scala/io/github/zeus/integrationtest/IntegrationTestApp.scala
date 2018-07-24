package io.github.zeus.integrationtest

import io.github.zeus.integrationtest.preparation.{PreparingParquetData, PreparingParquetDataArgs}
import io.github.zeus.integrationtest.verify.{VerifyArgs, VerifyCommand}
import org.rogach.scallop.ScallopConf

object IntegrationTestApp {
  def main(args: Array[String]): Unit = {
    val commandLine = new CommandLine(args)

    commandLine.subcommand match {
      case Some(args: VerifyArgs) =>
        new VerifyCommand(args).run
      case Some(args: PreparingParquetDataArgs) =>
        new PreparingParquetData(args).run
      case None =>
        System.exit(1)
    }
  }
}

class CommandLine(args: Array[String]) extends ScallopConf(args) {
  val compareResultSubCommand = new VerifyArgs()
  val preparingParquetDataArgs = new PreparingParquetDataArgs

  addSubcommand(compareResultSubCommand)
  addSubcommand(preparingParquetDataArgs)

  verify()
}
