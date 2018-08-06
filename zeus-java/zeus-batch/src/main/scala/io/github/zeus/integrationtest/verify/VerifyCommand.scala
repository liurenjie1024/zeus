package io.github.zeus.integrationtest.verify

import java.io.{FileOutputStream, PrintWriter}
import java.sql.DriverManager

import io.github.zeus.integrationtest.{Command, SqlElement}
import io.github.zeus.common.AutoManageResources._
import io.github.zeus.utils.template.TemplateEngine
import org.slf4j.{Logger, LoggerFactory}

class VerifyCommand(args: VerifyArgs) extends Command {
  private val LOG: Logger = LoggerFactory.getLogger(classOf[VerifyCommand])

  override def run: Unit = {
    val sqls = loadSqls
    compareResults(sqls)
  }

  private def loadSqls: Seq[SqlElement] = {
    LOG.info("Loading sqls...")

    val sqls = SqlElement.loadFile(args.sqlFilename())

    LOG.info("Sqls loaded")

    sqls
  }

  private def compareResults(sqls: Seq[SqlElement]): Unit = {
    LOG.info("Comparing results...")
    Class.forName(args.jdbcDriver())

    val conn = DriverManager.getConnection(args.jdbcUrl())
    val templateEngine = new TemplateEngine

    val drillEnv = Map("tableName" -> args.drillTableName())
    val zeusEnv = Map("tableName" -> args.zeusTableName())

    var (sameCounter, diffCounter, failureCoutner) = (0, 0, 0)
    val compareResults = List.newBuilder[CompareResult]

    conn.flatMap { c =>
      c.createStatement().flatMap { stat =>
        sqls.foreach { sql =>
          LOG.info(s"Comparing sql: ${sql.name}")

          val drillSql = templateEngine.transform(sql.sql, drillEnv)
          LOG.info(s"Drill sql is: [$drillSql]")

          val zeusSql = templateEngine.transform(sql.sql, zeusEnv)
          LOG.info(s"Zeus sql is: [$zeusSql]")

          var same = false
          var drillRS: JDBCResultSet = null
          var zeusRS: JDBCResultSet = null
          var error: Throwable = null

          try {
            stat.executeQuery(drillSql).flatMap { drillResultSet =>
              drillRS = JDBCResultSet.fromResultSet(drillResultSet)

              stat.executeQuery(zeusSql).flatMap { zeusResultSet =>
                zeusRS = JDBCResultSet.fromResultSet(zeusResultSet)

                same = if (sql.isOrdered) {
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
              error = t
              failureCoutner += 1
              LOG.error(s"Failed to compare ${sql.name}", t)
          }

          compareResults += CompareResult(sql, same, Option(drillRS), Option(zeusRS), Option(error))
        }
      }
    }

    new FileOutputStream(args.outputFilename()).flatMap { fos =>
      new PrintWriter(fos) { writer =>
        CompareResults(sameCounter, diffCounter, failureCoutner,
          compareResults.result()).output(writer)
      }
    }
  }
}

