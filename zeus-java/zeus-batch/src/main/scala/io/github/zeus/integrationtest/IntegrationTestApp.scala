package io.github.zeus.integrationtest

import org.rogach.scallop.{ScallopConf, Subcommand}

object IntegrationTestApp {
}

class CommandLine(args: Array[String]) extends ScallopConf(args) {
  val compareResultSubCommand = new Subcommand("verify")
}
