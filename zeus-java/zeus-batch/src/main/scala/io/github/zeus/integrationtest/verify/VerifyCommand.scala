package io.github.zeus.integrationtest.verify

import java.nio.file.{Files, Paths}
import java.sql.{Connection, Driver, DriverManager}

import io.circe.Json
import io.github.zeus.integrationtest.{Command, SqlElement}
import org.slf4j.{Logger, LoggerFactory}
import io.github.zeus.utils.AutoManageResources._
import io.circe.parser._
import io.github.zeus.utils.template.TemplateEngine
import org.apache.hadoop.conf.Configuration
import org.apache.hadoop.fs.FileSystem

class VerifyCommand(args: VerifyArgs) extends Command {
  private val LOG: Logger = LoggerFactory.getLogger(classOf[VerifyCommand])
  override def run: Unit = {

  }

  private def loadSqls: Seq[SqlElement] = {
    LOG.info("Loading sqls...")

    val sqls = SqlElement.loadFile(args.sqlFilename())

    LOG.info("Sqls loaded")

    sqls
  }

  private def compareResults(sqls: Seq[SqlElement]): Unit = {
    LOG.info("Comparing results...")
    Class.forName(args.jdbcDriverClassname())

    val conn = DriverManager.getConnection(args.jdbcUrl())
    val templateEngine = new TemplateEngine

    conn.flatMap { c =>
      c.createStatement().flatMap { stat =>
        sqls.foreach { sql =>
          LOG.info(s"Comparing sql: ${sql.name}")

          val drillSql = templateEngine.transform(sql.sql, Map("tableName" => ))

//          stat.executeQuery(sql.sql) { rs =>
//
//          }
        }
      }
    }
  }

  private def loadExpectedResult(name: String): Json = {
    val data = new String(
      Files.readAllBytes(Paths.get(s"${args.resultDir}/${name}.json}")),
      "UTF-8")
    parse(data) match {
      case Left(f) =>
        throw f
      case Right(json) =>
        json
    }
  }
}
