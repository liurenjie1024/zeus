package io.github.zeus.integrationtest.verify

import org.rogach.scallop.{ScallopOption, Subcommand}

class VerifyArgs extends Subcommand("verify") {
  val jdbcUrl: ScallopOption[String] = opt[String]("jdbcUrl", 'u', required = true)
  val jdbcDriverClassname = opt[String]("jdbcDriver", 'd', required = true)
  val sqlFilename: ScallopOption[String] = opt[String]("sqlFilename", 's', required = true)
  val resultDir: ScallopOption[String] = opt[String]("resultDir", 'r', required = true)
}
