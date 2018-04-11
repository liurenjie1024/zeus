pub mod block_input_stream;
pub mod column;
pub mod data_type;
mod simple_storage;
mod blizard_storage;
pub mod storage;
pub mod storage_factory;
pub mod storage_manager;

pub use self::block_input_stream::BlockInputStream;
pub use self::storage_manager::StorageManager;
pub use self::storage::Storage;
pub use self::storage::ScanContext;

#[derive(Debug)]
pub enum ErrorKind {
  InvalidHeader,
  InvalidFieldType,
  TableNotFound,
  InvalidStorageType,
  EOF,
}
