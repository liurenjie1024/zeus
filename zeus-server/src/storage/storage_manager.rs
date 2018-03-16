use std::sync::Arc;
use std::collections::HashMap;

use util::error::Result;
use storage::Storage;
use server::config::ZeusConfig;

pub struct StorageManager {
  tables: HashMap<i32, Arc<Storage>>,
}

unsafe impl Sync for StorageManager {}
unsafe impl Send for StorageManager {}

impl StorageManager {
  pub fn load(config: &ZeusConfig) -> Result<StorageManager> {
    unimplemented!()
  }

  pub fn get_table(
    &self,
    table_id: i32,
  ) -> Option<Arc<Storage>>
  {
    self.tables.get(&table_id).map(|s| s.clone())
  }
}
