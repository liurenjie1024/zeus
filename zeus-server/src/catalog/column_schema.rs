use std::sync::Arc;

use rpc::zeus_meta::ColumnType;
use rpc::zeus_meta::ZeusColumnSchema;

pub struct ColumnSchema {
  inner: Arc<ZeusColumnSchema>
}

impl ColumnSchema {
  pub fn new(zeus_column_schema: ZeusColumnSchema) -> ColumnSchema {
    ColumnSchema {
      inner: Arc::new(zeus_column_schema)
    }
  }

  pub fn get_id(&self) -> i32 {
    self.inner.get_id()
  }
  pub fn get_name(&self) -> String {
    self.inner.get_name().to_string()
  }
  pub fn get_type(&self) -> ColumnType {
    self.inner.get_column_type()
  }
}