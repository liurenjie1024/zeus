import subprocess
import os

SPARK_CMD = """$SPARK_HOME/bin/spark-submit \
  --master yarn-client \
  --class io.github.zeus.integrationtest.IntegrationTestApp \
  --conf spark.executor.memory=12G \
  --conf spark.task.maxFailures=1000 \
  --conf spark.yarn.max.executor.failures=2000 \
  --conf spark.dynamicAllocation.enable=false \
  --conf spark.executor.instances={instance_num} \
  --conf spark.sql.parquet.compression.codec=uncompressed \
  --conf spark.hadoop.mapreduce.output.fileoutputformat.compress=false \
  --conf spark.driver.memory=60G \
  --queue adhoc \
  --driver-java-options '-XX:+UseG1GC -XX:+PrintGC -XX:+PrintGCDetails -Xloggc:spark.driver.gc.log' \
  --driver-memory 16G \
  target/zeus-batch-0.0.1-SNAPSHOT.jar {args}"""

PARQUET_DIR = "zeus-data/parquet"


def run_spark(instance_num, args):
    env = {**os.environ, 'SPARK_HOME': '/opt/spark-2.1.0-cdh5.8.0'}
    cmd = SPARK_CMD.format(instance_num=instance_num, args=args)

    print("cmd is {}".format(cmd))
    subprocess.run(cmd, shell=True, env=env)


def prepare_parquet():
    instance_num = 100

    source_path = "/mvad/rawlog/dsp-charge/2018-07-17/09/dsp.charge.6.click/*"
    args = "preparing-parquet -s {source_path} -d {dest_path} -p 8 -l 10000".format(
        source_path=source_path,
        dest_path=PARQUET_DIR)

    run_spark(instance_num, args)


# def prepare_query_result():
#     class_name = "io.github.zeus.integrationtest.preparation.PrepareQueryResult"
#     instance_num = 8
#
#     sqls_file = "src/main/resources/sqls.xml"
#     parquet_file = "{}/1/*.parquet".format(PARQUET_DIR)
#     view_name = "realtimelog"
#
#     args = "-s {sqls_file} -p {parquet_file} -o {output_path} -n {view_name}".format(
#         sqls_file=sqls_file,
#         parquet_file=parquet_file,
#         output_path=QUERY_RESULT_DIR,
#         view_name=view_name)
#
#     run_spark(class_name, instance_num, args)


def verify_results():
    jdbc_url = "jdbc:drill:drillbit=qt9ss.prod.mediav.com:31010"
    jdbc_driver = "org.apache.drill.jdbc.Driver"
    sqls_filename = "src/main/resources/sqls.xml"
    drill_tablename = "dfs.\`*.parquet\`"
    zeus_tablename = "logs.realtimelog"
    output_filename = "results"

    args = ('verify --jdbcUrl "{jdbc_url}" --jdbcDriver "{jdbc_driver}" '
            '--sqlFilename "{sqls_filename}" '
            '--drillTablename "{drill_tablename}" --zeusTablename "{zeus_tablename}" '
            '--outputFilename "{output_filename}"').format(
        jdbc_url=jdbc_url, jdbc_driver=jdbc_driver, sqls_filename=sqls_filename,
        drill_tablename=drill_tablename, zeus_tablename=zeus_tablename,
        output_filename=output_filename)

    run_spark(0, args)


prepare_parquet()
# prepare_query_result()
# verify_results()
