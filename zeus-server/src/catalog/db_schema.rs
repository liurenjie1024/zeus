use std::sync::Arc;
use std::collections::HashMap;
use std::vec::Vec;

use rpc::zeus_meta::ZeusDBSchema;
use super::table_schema::TableSchema;

pub struct DBSchema {
  inner: Arc<ZeusDBSchema>,
  tables: HashMap<i32, Arc<TableSchema>>
}

impl DBSchema {
  pub fn new(zeus_db_schema: ZeusDBSchema) -> DBSchema {
    let mut tables = HashMap::new();
    zeus_db_schema.get_tables()
      .values()
      .map(|t| Arc::new(TableSchema::new(t.clone())))
      .for_each(|t| {
        tables.insert(t.get_id(), t);
      });

    DBSchema {
      inner: Arc::new(zeus_db_schema),
      tables
    }
  }

  pub fn get_id(&self) -> i32 {
    self.inner.get_id()
  }

  pub fn get_table(&self, table_id: i32) -> Option<Arc<TableSchema>> {
    self.tables.get(&table_id)
      .map(|t| t.clone())
  }

  pub(super) fn list_table_ids(&self) -> Vec<i32>  {
    self.tables.values()
      .clone()
      .map(|t| t.get_id())
      .collect()
  }
}
