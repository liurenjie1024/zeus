use std::vec::Vec;

use futures::sync::oneshot::channel;

use rpc::zeus_data_grpc::ZeusDataService;
use rpc::zeus_data::QueryRequest;
use rpc::zeus_data::QueryResult;
use rpc::zeus_data::RowResult;
use rpc::zeus_data::StatusCode;

use exec::DAGExecutor;
use error_chain::ChainedError;
use util::errors::*;

use grpcio::RpcContext;
use grpcio::UnarySink;
use protobuf::RepeatedField;
use futures::future::Future;


use server::ServerContext;

pub type Rows = Vec<RowResult>;

#[derive(Clone)]
pub struct DataService {
  server_context: ServerContext,
}

impl DataService {
  pub fn new(server_context: ServerContext) -> DataService {
    DataService {
      server_context,
    }
  }
}

impl ZeusDataService for DataService {
  fn query(
    &self,
    ctx: RpcContext,
    req: QueryRequest,
    sink: UnarySink<QueryResult>,
  )
  {
    let plan_id = req.get_plan().get_plan_id().to_string();
    debug!("Begin to query, plan id is: {}", plan_id);

    let (sender, receiver) = channel();

    let task = DAGExecutor::task(req, sender, self.server_context.clone());

    //TODO: Handle error here.
    self.server_context.query_scheduler.submit(task).ok().unwrap();

    let future = receiver
      .map_err(|e| Error::from(e))
      .map(|res| {
        let mut result = QueryResult::new();

        match res {
          Ok(rows) => {
            result.set_code(StatusCode::OK);
            result.set_rows(RepeatedField::from_vec(rows));
          },
          Err(err) => {
            info!("Query failed: {}", err.display_chain());
            result.set_code(err.into())
          },
        }
        result
      })
      .and_then(|res| sink.success(res).map_err(|e1| Error::from(e1)))
      .map_err(move |e| {
        error!("Query failed, plan id: {}, error: {:?}", plan_id, e);
      });

    ctx.spawn(future);
  }
}
