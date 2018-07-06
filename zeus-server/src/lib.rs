#![feature(iterator_try_fold)]
#![feature(fnbox)]
#![feature(box_syntax)]
#![feature(try_from)]
#![feature(vec_resize_default)]
#![recursion_limit="128"]

extern crate bytes;
extern crate futures;
extern crate futures_cpupool;
extern crate grpcio;
#[macro_use]
extern crate log;
extern crate prometheus;
extern crate protobuf;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate toml;
#[macro_use]
extern crate error_chain;
extern crate arrow;
extern crate byteorder;
extern crate parquet;

pub mod exec;
pub mod rpc;
pub mod scheduler;
pub mod server;
pub mod storage;
pub mod util;
pub mod catalog;
