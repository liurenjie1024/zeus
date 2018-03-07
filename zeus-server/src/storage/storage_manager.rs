use std::sync::Arc;
use std::collections::HashMap;

use util::error::Result;
use storage::Storage;
use storage::CatalogManager;

#[derive(Clone, Debug)]
pub struct StorageManagerConfig {
    // pending appendable segment num per table
    pub max_pending_segment_num: usize,
    // maximum size of appendable segment
    pub max_appendable_segment_size: usize,
    // database directory
    pub path: String
}


pub struct StorageManager {
    tables: HashMap<i32, Arc<Storage>>,
    catalog_manager: Arc<CatalogManager>
}

impl StorageManager {
    pub fn load(engine_config: &StorageManagerConfig) -> Result<Box<StorageManager>> {
        unimplemented!()
    }

    pub fn get_table(&self, table_id: i32) -> Option<Arc<Storage>> {
        self.tables.get(&table_id)
            .map(|s| s.clone())
    }

    pub fn get_catalog_manager(&self) -> Arc<CatalogManager> {
        self.catalog_manager.clone()
    }
}
