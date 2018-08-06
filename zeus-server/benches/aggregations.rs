#[macro_use]
extern crate criterion;
extern crate zeus;
extern crate futures;

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use criterion::Criterion;
use criterion::Benchmark;

use futures::sync::oneshot::channel;

use zeus::server::ServerContext;
use zeus::server::server::ZeusServer;
use zeus::server::config::ZeusConfig;
use zeus::rpc::zeus_data::QueryRequest;
use zeus::exec::DAGExecutor;
use zeus::test_utils::parser::ProtobufJsonParser;
use zeus::test_utils::file_in_project_dir;
use zeus::util::errors::*;



struct QueryExecutionTest {
  server_context: ServerContext,
  query_request: QueryRequest
}

impl QueryExecutionTest {
  pub fn new<P: AsRef<Path>>(config_file: P, query_request: QueryRequest)
    -> Result<QueryExecutionTest> {
    let config = Arc::new(ZeusConfig::from_toml(config_file)?);
    let server_context = ZeusServer::create_server_context(config)?;

    Ok(QueryExecutionTest {
      server_context,
      query_request
    })
  }

  pub fn run(&self) {
    let (sender, _receiver) = channel();
    let query_task = DAGExecutor::task(self.query_request.clone(),
      sender,
      self.server_context.clone());

    query_task.run();
  }
}

fn sum_benchmark(c: &mut Criterion) {
  let config_filename = file_in_project_dir("benches/data/test-config.toml");

  let query_request: QueryRequest = {
    let parser = ProtobufJsonParser::new("../zeus-java/zeus-util/target/zeus-util-0.0.1-SNAPSHOT.jar");
    parser.parse(file_in_project_dir("benches/sum_query_request.json") ,
      "io.github.zeus.rpc.QueryRequest")
      .unwrap()
  };

  let test = QueryExecutionTest::new(config_filename, query_request).unwrap();

  let benchmark = Benchmark::new("sum", move |b| b.iter(|| test.run()))
    .sample_size(5)
    .warm_up_time(Duration::from_secs(300));

  c.bench("sum", benchmark);
}

criterion_group!(benches, sum_benchmark);
criterion_main!(benches);