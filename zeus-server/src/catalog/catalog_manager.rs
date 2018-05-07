use std::sync::Arc;
use std::collections::HashMap;
use std::vec::Vec;
use std::default::Default;

use rpc::zeus_meta::ZeusCatalog;

use super::table_schema::TableSchema;
use super::db_schema::DBSchema;

pub struct CatalogManager {
  dbs: HashMap<i32, Arc<DBSchema>>,
  table_to_db: HashMap<i32, i32>
}

//TODO: Optimize this
pub struct TableIds {
  pos: usize,
  ids: Vec<i32>
}

impl Iterator for TableIds {
  type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
    let ret = self.ids.get(self.pos)
      .map(|&x| x);
    self.pos += 1;
    ret
  }
}

impl Default for CatalogManager {
  fn default() -> Self {
    CatalogManager {
      dbs: HashMap::new(),
      table_to_db: HashMap::new()
    }
  }
}


impl CatalogManager {
  pub fn new(catalog: ZeusCatalog) -> CatalogManager {
    let mut dbs = HashMap::new();
    catalog.get_db_schemas()
      .iter()
      .map(|db| Arc::new(DBSchema::new(db.clone())) )
      .for_each(|db| {
        dbs.insert(db.get_id(), db);
      });

    let mut table_to_db = HashMap::new();
    catalog.get_db_schemas()
      .iter()
      .for_each(|db| {
        db.get_tables()
          .values()
          .for_each(|t| {
            table_to_db.insert(t.get_id(), db.get_id());
          })
      });

    CatalogManager {
      dbs,
      table_to_db
    }
  }

  pub fn get_table_schema(
    &self,
    table_id: i32,
  ) -> Option<Arc<TableSchema>> {
    self.table_to_db.get(&table_id)
      .and_then(|db_id| self.dbs.get(&db_id))
      .and_then(|db| db.get_table(table_id))
  }

  pub fn list_table_ids(&self) -> TableIds {
    let ids = self.dbs.values()
      .clone()
      .flat_map(|db| db.list_table_ids())
      .collect();

    TableIds {
      pos: 0,
      ids
    }
  }
}