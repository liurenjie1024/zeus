use std::sync::Arc;
use std::collections::HashMap;

use error_chain::ChainedError;

use util::errors::*;
use catalog::CatalogManager;
use storage::Storage;
use storage::storage_factory::StorageFactory;
use server::config::ZeusConfig;

pub struct StorageManager {
  tables: HashMap<i32, Arc<Storage>>,
}

unsafe impl Sync for StorageManager {}
unsafe impl Send for StorageManager {}

impl StorageManager {
  pub fn load(config: &ZeusConfig, catalog_manager: Arc<CatalogManager>) -> Result<StorageManager> {
    let mut tables = HashMap::new();
    catalog_manager.list_table_ids()
      .map(|table_id| catalog_manager.get_table_schema(table_id).unwrap())
      .for_each(|table_schema| {
        let storage = StorageFactory::create_storage(
          table_schema.get_id(),
          table_schema.get_table_engine().as_str(),
          config);
        match storage {
          Ok(s) => {
            tables.insert(s.get_id(), s);
          },
          Err(e) => {
            info!("Failed to load storage: {}", e.display_chain())
          }
        }
      });

    Ok(StorageManager {
      tables
    })
  }

  pub fn get_table(
    &self,
    table_id: i32,
  ) -> Option<Arc<Storage>> {
    self.tables.get(&table_id).map(|s| s.clone())
  }
}
