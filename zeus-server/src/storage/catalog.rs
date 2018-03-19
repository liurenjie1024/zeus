use std::sync::Arc;
use std::iter::Iterator;

use rpc::zeus_meta::FieldType;
use util::error::Result;
use server::config::ZeusConfig;

pub trait ColumnSchema {
  fn get_id(&self) -> i32;
  fn get_name(&self) -> String;
  fn get_type(&self) -> FieldType;
}

pub trait TableSchema {
  fn get_table_engine(&self) -> String;
  fn get_id(&self) -> i32;
  fn get_column_schema(
    &self,
    column_id: i32,
  ) -> Option<Arc<ColumnSchema>>;
}

pub trait CatalogManager: Send + Sync {
  fn get_table_schema(
    &self,
    table_id: i32,
  ) -> Option<Arc<TableSchema>>;

  fn list_table_ids(&self) -> Box<Iterator<Item=i32>>;
}

pub fn load(_config: &ZeusConfig) -> Result<Arc<CatalogManager>> {
  unimplemented!()
}
