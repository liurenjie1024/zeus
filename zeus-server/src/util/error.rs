use ::grpcio::Error as GrpcError;
use std::result;

#[derive(Debug)]
pub enum Error {
    Grpc(GrpcError)
}

pub type Result<T> = result::Result<T, Error>;
