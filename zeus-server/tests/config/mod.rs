use zeus::test_utils::file_in_project_dir;
use zeus::server::config::*;

#[test]
fn test_read_config() {
  let mut zeus_config = ZeusConfig::default();

  zeus_config.server = ServerConfig {
    server_addr: "0.0.0.0:8899".to_string(),
    grpc_concurrency: 10,
    grpc_concurrent_stream: 10,
    grpc_stream_init_window_size: 20,
    grpc_max_send_msg_len: 10*1024*1024,
    grpc_max_recv_msg_len: 10*1024*1024
  };

  zeus_config.storage = StorageConfig {
    root_path: "zeus/data".to_string(),
    schema_filename: "test.schema".to_string()
  };

  zeus_config.query = QuerySchedulerConfig {
    worker_size: 4
  };

  let config_file = ZeusConfig::from_toml(file_in_project_dir("tests/config/test-config.toml"));

  assert!(config_file.is_ok());
  assert_eq!(zeus_config, config_file.ok().unwrap());
}

