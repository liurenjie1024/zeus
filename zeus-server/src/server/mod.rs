pub mod server;
mod data_service;
mod meta_service;

use std::sync::Arc;

use util::error::Result;


pub const MAX_GRPC_RECV_MSG_SIZE: usize = 10*1024*1024;
pub const MAX_GRPC_SEND_MSG_SIZE: usize = 10*1024*1024;

pub struct Config {
    pub server_addr: String,
    pub grpc_concurrency: usize,
    pub grpc_concurrent_stream: usize,
    pub grpc_stream_init_window_size: usize,
    pub grpc_max_send_msg_len: usize,
    pub grpc_max_recv_msg_len: usize,

    pub query_concurrency: usize
}

pub struct ServerContext {}

