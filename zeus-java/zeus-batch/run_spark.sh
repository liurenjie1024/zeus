#!/usr/bin/env bash
export SPARK_HOME=/opt/spark-2.1.0-cdh5.8.0

echo "Begin to run spark"
$SPARK_HOME/bin/spark-submit \
  --master yarn-client \
  --class io.github.zeus.batch.format.parquet.ParquetReader \
  --conf spark.executor.memory=12G \
  --conf spark.task.maxFailures=1000 \
  --conf spark.yarn.max.executor.failures=2000 \
  --conf mapreduce.output.fileoutputformat.compress=false \
  --conf spark.dynamicAllocation.enable=false \
  --conf spark.executor.instances=200 \
  --conf spark.driver.memory=60G \
  --queue adhoc \
  --driver-java-options "-XX:+UseG1GC -XX:+PrintGC -XX:+PrintGCDetails -Xloggc:spark.driver.gc.log" \
  --driver-memory 16G \
  target/zeus-batch-0.0.1-SNAPSHOT.jar "$@"

#  --conf spark.dynamicAllocation.initialExecutors=1000 \
#  --conf spark.dynamicAllocation.minExecutors=1000 \
#  --conf spark.dynamicAllocation.maxExecutors=2000 \
