use std::convert::Into;
use std::result;

use futures::sync::oneshot::Canceled;

use grpcio::Error as GrpcError;
use protobuf::error::ProtobufError;
use serde_json::error::Error as SerdeJsonError;
use std::io::Error as StdIoError;
use storage::ErrorKind as DBErrorKind;
use scheduler::ErrorKind as SchedulerErrorKind;
use rpc::zeus_data::StatusCode;

quick_error! {
#[derive(Debug)]
pub enum Error {
  Grpc(grpc_error: GrpcError) {
    from()
  }
  IoError(err: StdIoError) {
    from()
  }
  PBError(err: ProtobufError) {
    from()
  }
  JsonSerdeError(err: SerdeJsonError) {
    from()
  }
  DBError(inner: DBErrorKind) {
  }
  SchedulerError(inner: SchedulerErrorKind) {
  }
  ServerError {
    from(Canceled)
  }
}
}

pub type Result<T> = result::Result<T, Error>;

impl Into<StatusCode> for Error {
  fn into(self) -> StatusCode {
    unimplemented!()
  }
}
