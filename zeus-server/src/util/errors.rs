use std::convert::Into;

use rpc::zeus_expr::AggFuncId;
use rpc::zeus_data::StatusCode;

use storage;
use scheduler;

error_chain! {
  types {
    Error, ErrorKind, ResultExt, Result;
  }

  links {
  }

  foreign_links {
    Grpc(::grpcio::Error);
    Io(::std::io::Error);
    Protobuf(::protobuf::error::ProtobufError);
    JsonSerde(::serde_json::error::Error);
    FutureOneShotCancelled(::futures::sync::oneshot::Canceled);
    TomlSer(::toml::ser::Error);
    TomlDe(::toml::de::Error);
    UTF8(::std::string::FromUtf8Error);
  }

  errors {
    DB(inner: storage::ErrorKind) {
    }
    Scheduler(inner: scheduler::ErrorKind) {
    }
    ColumnNameNotFound(column_name: String) {
      description("invalid column name")
      display("invalid column name: '{}'", column_name)
    }
    UnableToCompare (msg: String) {
    }
    BlockTypeNotMatch(msg: String) {
    }
    IndexOutOfBound(pos: usize, bound: usize) {
      description("Index out of bound")
      display("Index out of bound, index is {}, bound is {}", pos, bound)
    }
    EmptyAggregator(agg_func_id: AggFuncId) {
      description("Aggregator has no data")
      display("Aggregator {:?} has no data", agg_func_id)
    }
    EOF {
    }
  }
}

impl Into<StatusCode> for Error {
  fn into(self) -> StatusCode {
    StatusCode::FAILED
  }
}
