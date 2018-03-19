use std::sync::Arc;

use rpc::zeus_meta::FieldType;
use util::error::Result;
use server::config::ZeusConfig;

pub trait ColumnSchema {
  fn get_id(&self) -> i32;
  fn get_name(&self) -> String;
  fn get_type(&self) -> FieldType;
}

pub trait TableSchema {
  fn get_id(&self) -> i32;
  fn get_column_schema(
    &self,
    column_id: i32,
  ) -> Option<Arc<ColumnSchema>>;
}

pub trait CatalogManager: Send + Sync {
  fn get_table_schema(
    &self,
    db_id: i32,
    table_id: i32,
  ) -> Option<Arc<TableSchema>>;
}

pub fn load(_config: &ZeusConfig) -> Result<Arc<CatalogManager>> {
  unimplemented!()
}
