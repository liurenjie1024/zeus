use std::convert::Into;
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
  }
}

impl Into<StatusCode> for Error {
  fn into(self) -> StatusCode {
    StatusCode::FAILED
  }
}
