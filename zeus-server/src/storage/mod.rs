pub mod block_input_stream;
pub mod catalog;
pub mod column;
pub mod data_type;
mod simple_storage;
pub mod storage;
pub mod storage_factory;
pub mod storage_manager;

use util::error::Result;

pub use self::block_input_stream::BlockInputStream;
pub use self::storage_manager::StorageManager;
pub use self::storage::Storage;
pub use self::storage::ScanContext;
pub use self::catalog::CatalogManager;

#[derive(Debug)]
pub enum ErrorKind {
  InvalidHeader,
  InvalidFieldType,
  TableNotFound,
  EOF,
}

pub type StorageResult = Result<i32>;
