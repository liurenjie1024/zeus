use std::sync::Arc;

use util::error::Result;
use storage::Storage;

struct StorageFactory {}

impl StorageFactory {
  pub fn create_storage(
    table_id: i32,
    storage_type: &str,
  ) -> Result<Arc<Storage>>
  {
    unimplemented!()
  }
}
