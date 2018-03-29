use std::sync::Arc;

use util::errors::*;
use storage::ErrorKind as DBErrorKind;
use server::config::ZeusConfig;
use storage::Storage;
use storage::simple_storage::SimpleTable;

#[allow(dead_code)]
pub struct StorageFactory {}

impl StorageFactory {
  #[allow(dead_code)]
  pub fn create_storage(
    table_id: i32,
    storage_type: &str,
    config: &ZeusConfig
  ) -> Result<Arc<Storage>> {
    match storage_type {
      "simple" => Ok(Arc::new(SimpleTable::new(&config.storage, table_id)?)),
      s => {
        error!("Unrecognized storage type: {}", s);
        bail!(ErrorKind::DB(DBErrorKind::InvalidStorageType))
      }
    }
  }
}
