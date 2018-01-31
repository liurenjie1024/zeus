use rpc::zeus_data_grpc::ZeusDataService;
use rpc::zeus_data::QueryRequest;
use rpc::zeus_data::QueryResult;
use rpc::zeus_data::RowResult;
use rpc::zeus_data::ColumnValue;

use ::grpcio::RpcContext;
use ::grpcio::UnarySink;
use ::futures::future::Future;

#[derive(Clone)]
pub struct DataService {
}

impl DataService {
    pub fn new() -> DataService {
        DataService {

        }
    }
}

impl ZeusDataService for DataService {
    fn query(&self, ctx: RpcContext, req: QueryRequest, sink: UnarySink<QueryResult>) {
        println!("request!");
        let mut result = QueryResult::new();
        let mut row = RowResult::new();
        let mut column = ColumnValue::new();
        column.set_string_value("ok".to_string());
        row.mut_columns().push(column);
        result.mut_rows().push(row);


        let future = sink.success(result)
            .map(|_| ())
            .map_err(|_| ());

        ctx.spawn(future);
    }
}
