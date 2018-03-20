use std::convert::AsRef;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use toml::from_str as toml_from_str;

use util::error::Result;
use util::error::Error;

#[derive(Default,Serialize,Deserialize,Debug,Clone, Eq, PartialEq)]
pub struct ServerConfig {
  pub server_addr: String,
  pub grpc_concurrency: usize,
  pub grpc_concurrent_stream: usize,
  pub grpc_stream_init_window_size: usize,
  pub grpc_max_send_msg_len: usize,
  pub grpc_max_recv_msg_len: usize,
}

#[derive(Default,Serialize,Deserialize,Debug,Clone, Eq, PartialEq)]
pub struct StorageConfig {
  /// Root storage directory
  pub root_path: String,
}

#[derive(Default,Serialize,Deserialize,Debug,Clone, Eq, PartialEq)]
pub struct QuerySchedulerConfig {
  pub worker_size: usize,
}

#[derive(Default,Serialize,Deserialize,Debug,Clone, Eq, PartialEq)]
pub struct ZeusConfig {
  pub server: ServerConfig,
  pub storage: StorageConfig,
  pub query: QuerySchedulerConfig,
}

impl ZeusConfig {
  pub fn validate(&self) -> Result<()> {
    Ok(())
  }

  pub fn from_toml<P: AsRef<Path>>(path: P) -> Result<Self> {
    File::open(path)
      .map_err(|e| Error::from(e))
      .and_then(|mut f| {
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let config = toml_from_str(&s)?;
        Ok(config)
      })
  }
}
