package io.github.zeus.integrationtest.verify

import org.rogach.scallop.{ScallopOption, Subcommand}

class VerifyArgs extends Subcommand("verify") {
  val jdbcUrl: ScallopOption[String] = opt[String]("jdbcUrl", 'u', required = true)
  val jdbcDriverClassname: ScallopOption[String] = opt[String]("jdbcDriver", 'd', required = true)
  val sqlFilename: ScallopOption[String] = opt[String]("sqlFilename", 's', required = true)
  val drillTableName: ScallopOption[String] = opt[String]("drillTablename", 'r', required = true)
  val zeusTableName: ScallopOption[String] = opt[String]("zeusTablename", 'z', required = true)
}
