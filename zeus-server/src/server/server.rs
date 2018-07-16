use std::sync::Arc;
use std::net::SocketAddr;
use std::str::FromStr;

use grpcio::Server as GrpcServer;
use grpcio::EnvBuilder;
use grpcio::ChannelBuilder;
use grpcio::ServerBuilder;

use util::errors::*;
use server::config::ZeusConfig;
use server::config::ServerConfig;
use super::data_service::DataService;
use storage::StorageManager;
use rpc::zeus_data_grpc::create_zeus_data_service;
use server::ServerContext;
use catalog::load as load_catalog_manager;
use scheduler::build as build_scheduler;

#[allow(dead_code)]
pub struct ZeusServer {
  server: GrpcServer,
  context: ServerContext,
}

impl ZeusServer {
  pub fn new(config: Arc<ZeusConfig>) -> Result<ZeusServer> {
    let catalog_manager = load_catalog_manager(&*config)?;
    let storage_manager = Arc::new(StorageManager::load(&*config, catalog_manager.clone())?);
    let query_scheduler = build_scheduler("query", &*config)?;

    let context = ServerContext::new(storage_manager, catalog_manager, query_scheduler);

    Ok(ZeusServer {
      server: ZeusServer::create_grpc_server(&config.server, context.clone())?,
      context,
    })
  }

  fn create_grpc_server(
    config: &ServerConfig,
    context: ServerContext,
  ) -> Result<GrpcServer>
  {
    let env =
      Arc::new(EnvBuilder::new().cq_count(config.grpc_concurrency).name_prefix("grpc-cq").build());
    let data_service = DataService::new(context.clone());

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

    Ok(grpc_server)
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
