extern crate zeus;

use zeus::server::server::ZeusServer;
use std::sync::Arc;
use std::io;

fn main() {
//    let config = Arc::new(Config {
//        server_addr: "127.0.0.1:7788".to_string(),
//        grpc_concurrency: 4 as usize,
//        grpc_concurrent_stream: 10 as usize,
//        grpc_stream_init_window_size: 20 as usize,
//        grpc_max_send_msg_len: zeus::server::MAX_GRPC_SEND_MSG_SIZE,
//        grpc_max_recv_msg_len: zeus::server::MAX_GRPC_RECV_MSG_SIZE,
//
//        query_concurrency: 10 as usize
//    });
//
//    let mut server = ZeusServer::new(config.clone())
//        .unwrap();
//    println!("Starting server...");
//    server.start();
//    println!("Starting running...");
//
//    let mut buf = String::new();
//    let stdin = io::stdin();
//    stdin.read_line(&mut buf);

//    println!("Stopping server...");
//    server.stop();
//    println!("Starting stopped...");
}