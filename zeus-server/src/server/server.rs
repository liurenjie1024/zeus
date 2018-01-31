use std::sync::Arc;
use std::net::SocketAddr;
use std::str::FromStr;

use ::grpcio::Environment;
use ::grpcio::Server as GrpcServer;
use ::grpcio::EnvBuilder;
use ::grpcio::ChannelBuilder;
use ::grpcio::ServerBuilder;


use util::error::Result;
use super::Config;
use super::data_service::DataService;
use rpc::zeus_data_grpc::create_zeus_data_service;

pub struct ZeusServer {
    env: Arc<Environment>,
    server: GrpcServer
}

impl ZeusServer {
    pub fn new(config: Arc<Config>) -> Result<ZeusServer> {
        let env = Arc::new(
            EnvBuilder::new()
                .cq_count(config.grpc_concurrency)
                .name_prefix("grpc-cq")
                .build()
        );

        let data_service = DataService::new();

        let socket_addr = SocketAddr::from_str(&config.server_addr).unwrap();
        let ip_str = format!("{}", socket_addr.ip());
        let channel_args = ChannelBuilder::new(env.clone())
            .max_concurrent_stream(config.grpc_concurrent_stream)
            .max_receive_message_len(config.grpc_max_recv_msg_len)
            .max_send_message_len(config.grpc_max_send_msg_len)
            .stream_initial_window_size(config.grpc_stream_init_window_size)
            .build_args();

        let grpc_server = ServerBuilder::new(env.clone())
            .channel_args(channel_args)
            .register_service(create_zeus_data_service(data_service))
            .bind(ip_str, socket_addr.port())
            .build()
            .unwrap();

        Ok(ZeusServer {
            env: env.clone(),
            server: grpc_server
        })
    }

    pub fn start(&mut self) -> Result<()> {
        self.server.start();
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.server.shutdown();
        Ok(())
    }

}