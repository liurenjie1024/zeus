use ::grpcio::Error as GrpcError;
use protobuf::error::ProtobufError;
use std::result;

quick_error! {
#[derive(Debug)]
pub enum Error {
    Grpc(grpc_error: GrpcError) {
        from()
    }
    IoError(err: std::io::Error) {
        from()
    }
    PBError(err: ProtobufError) {
        from()
    }
}
}


pub type Result<T> = result::Result<T, Error>;
