import subprocess
import os

SPARK_CMD = """$SPARK_HOME/bin/spark-submit \
  --master yarn-client \
  --class {class_name} \
  --conf spark.executor.memory=12G \
  --conf spark.task.maxFailures=1000 \
  --conf spark.yarn.max.executor.failures=2000 \
  --conf spark.dynamicAllocation.enable=false \
  --conf spark.executor.instances={instance_num} \
  --conf spark.sql.parquet.compression.codec=uncompressed \
  --conf spark.driver.memory=60G \
  --queue adhoc \
  --driver-java-options '-XX:+UseG1GC -XX:+PrintGC -XX:+PrintGCDetails -Xloggc:spark.driver.gc.log' \
  --driver-memory 16G \
  target/zeus-batch-0.0.1-SNAPSHOT.jar {args}"""

PARQUET_DIR = "zeus-data/parquet"
QUERY_RESULT_DIR = "zeus-data/result"


def run_spark(class_name, instance_num, args):
    env = {**os.environ, 'SPARK_HOME': '/opt/spark-2.1.0-cdh5.8.0'}
    cmd = SPARK_CMD.format(class_name=class_name, instance_num=instance_num, args=args)

    subprocess.run(cmd, shell=True, env=env)


def prepare_parquet():
    class_name = "io.github.zeus.integrationtest.preparation.PrepareParquetData"
    instance_num = 100

    source_path = "/mvad/rawlog/dsp-charge/2018-07-17/09/dsp.charge.6.click/*"
    args = "-s {source_path} -d {dest_path} -p 8 -n 10000".format(
        source_path=source_path,
        dest_path=PARQUET_DIR)

    run_spark(class_name, instance_num, args)


prepare_parquet()
