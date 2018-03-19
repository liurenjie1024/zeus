use std::sync::Arc;

use util::error::Result;
use storage::Storage;

#[allow(dead_code)]
struct StorageFactory {}

impl StorageFactory {
  #[allow(dead_code)]
  pub fn create_storage(
    _table_id: i32,
    _storage_type: &str,
  ) -> Result<Arc<Storage>>
  {
    unimplemented!()
  }
}
