package io.github.zeus.integrationtest.verify

import java.nio.file.{Files, Paths}
import java.sql._

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

    val drillEnv = Map("tableName" -> args.drillTableName())
    val zeusEnv = Map("tableName" -> args.zeusTableName())

    var (sameCounter, diffCounter, failureCoutner) = (0, 0, 0)

    conn.flatMap { c =>
      c.createStatement().flatMap { stat =>
        sqls.foreach { sql =>
          LOG.info(s"Comparing sql: ${sql.name}")

          val drillSql = templateEngine.transform(sql.sql, drillEnv)
          LOG.info(s"Drill sql is: [$drillSql]")

          val zeusSql = templateEngine.transform(sql.sql, zeusEnv)
          LOG.info(s"Zeus sql is: [$zeusSql]")
          try {
            stat.executeQuery(drillSql) { drillResultSet =>
              stat.executeQuery(zeusSql) { zeusResultSet =>
                val drillRS = JDBCResultSet.fromResultSet(drillResultSet)
                val zeusRS = JDBCResultSet.fromResultSet(zeusResultSet)

                val same = if (sql.isOrdered) {
                  JDBCResultSet.compareOrdered(drillRS, zeusRS)
                } else {
                  JDBCResultSet.compareUnordered(drillRS, zeusRS)
                }

                if (same) {
                  sameCounter += 1
                  LOG.info(s"${sql.name} is same.")
                } else {
                  diffCounter += 1
                  LOG.info(s"${sql.name} is different.")
                }
              }
            }
          } catch {
            case t: Throwable =>
              failureCoutner += 1
              LOG.error(s"Failed to compare ${sql.name}", t)
          }
        }
      }
    }
  }

//  private def compareResultSet(expectedRS: ResultSet,
//    actualRS: ResultSet, isOrdered: Boolean): Boolean = {
//
//  }
//  def compareResultSetMetadata(
//    expectedRSMetadata: ResultSetMetaData,
//    actualRSMetadata: ResultSetMetaData): Boolean = {
//
//    if (expectedRSMetadata.getColumnCount != actualRSMetadata.getColumnCount) {
//    }
//  }
}

object VerifyCommand {
}
