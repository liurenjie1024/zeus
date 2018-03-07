pub mod column;
pub mod data_type;
pub mod block_input_stream;
mod storage_manager;
mod storage_factory;
mod catalog;
mod storage;
mod simple_storage;


use std::clone::Clone;
use std::boxed::Box;
use std::vec::Vec;
use std::fmt::Display;
use std::path::Path;
use std::sync::Arc;
use std::collections::HashMap;

use storage::column::Column;
use util::error::Result;
use util::cow_ptr::CowPtr;
use exec::Block;
use rpc::zeus_data::ScanNode;
use rpc::zeus_meta::ZeusTableSchema;

pub use self::block_input_stream::BlockInputStream;
pub use self::storage_manager::StorageManager;
pub use self::storage_manager::StorageManagerConfig;
pub use self::storage::Storage;
pub use self::storage::ScanContext;
pub use self::catalog::CatalogManager;




#[derive(Debug)]
pub enum ErrorKind {
    InvalidHeader,
    InvalidFieldType,
    TableNotFound,
    EOF
}

pub type StorageResult = Result<i32>;


