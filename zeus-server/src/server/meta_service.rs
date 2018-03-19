use rpc::zeus_meta_grpc::ZeusMetaService;
use rpc::zeus_meta::GetSchemaRequest;
use rpc::zeus_meta::GetSchemaResponse;

use grpcio::RpcContext;
use grpcio::UnarySink;

#[derive(Clone)]
pub struct MetaService {}

impl ZeusMetaService for MetaService {
  fn get_db_schema(
    &self,
    _ctx: RpcContext,
    _req: GetSchemaRequest,
    _sink: UnarySink<GetSchemaResponse>,
  )
  {
    unimplemented!()
  }
}
