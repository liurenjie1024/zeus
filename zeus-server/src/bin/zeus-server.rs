extern crate zeus;
extern crate clap;
#[macro_use]
extern crate log;
extern crate error_chain;
extern crate log4rs;


use std::sync::Arc;
use std::process;

use clap::{App, Arg};
use error_chain::ChainedError;
use zeus::server::config::ZeusConfig;
use zeus::server::server::ZeusServer;

macro_rules! fatal {
  ($lvl:expr, $($arg:tt)+) => ({
    error!($lvl, $($arg)+);
    process::exit(1)
  })
}

#[allow(unused_must_use)]
fn main() {
  let matches = App::new("Zeus Server")
    .about("A time-series database written in rust!")
    .author("liurenjie1024 <liurenjie2008@gmail.com>")
    .arg(Arg::with_name("config")
      .short("c")
      .long("config")
      .value_name("CONFIG_FILE")
      .required(true)
      .takes_value(true))
    .arg(Arg::with_name("log")
      .short("l")
      .long("log")
      .value_name("LOG_CONFIG_FILE")
      .takes_value(true))
    .get_matches();

  let log_config_file = matches.value_of("log")
    .unwrap_or("log4rs.yml");
  log4rs::init_file(log_config_file, Default::default())
    .unwrap_or_else(|e| panic!("Failed to init logger: {}", e));


  let config = ZeusConfig::from_toml(matches.value_of("config").unwrap())
    .unwrap_or_else(|e| fatal!("Failed to parse config: {}", e));

  let mut server = ZeusServer::new(Arc::new(config))
    .unwrap_or_else(|e| fatal!("Failed to create server: {}", e.display_chain()));

  server.start();

  info!("zeus server started...");

  let mut buf = String::new();
  let stdin = std::io::stdin();
  stdin.read_line(&mut buf);

  info!("stopping zeus server...");
  server.stop();
  info!("zeus server stopped...");
}

