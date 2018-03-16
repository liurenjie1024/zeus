use util::error::Result;

#[derive(Default)]
pub struct ServerConfig {
  pub server_addr: String,
  pub grpc_concurrency: usize,
  pub grpc_concurrent_stream: usize,
  pub grpc_stream_init_window_size: usize,
  pub grpc_max_send_msg_len: usize,
  pub grpc_max_recv_msg_len: usize,

  pub query_concurrency: usize,
}

#[derive(Default)]
pub struct StorageConfig {
  /// Root dir storage
  pub path: String,
}

#[derive(Default)]
pub struct QuerySchedulerConfig {
  pub worker_size: usize,
}

#[derive(Default)]
pub struct ZeusConfig {
  pub server_config: ServerConfig,
  pub storage_config: StorageConfig,
  pub query_config: QuerySchedulerConfig,
}

impl ZeusConfig {
  pub fn validate(&self) -> Result<()> {
    Ok(())
  }
}
