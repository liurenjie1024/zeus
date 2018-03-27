use std::sync::Arc;
use std::collections::HashMap;

use rpc::zeus_meta::ZeusTableSchema;
use super::column_schema::ColumnSchema;


pub struct TableSchema {
  inner: Arc<ZeusTableSchema>,
  column_schemas: HashMap<i32, Arc<ColumnSchema>>
}

impl TableSchema {
  pub fn new(zeus_table_schema: ZeusTableSchema) -> TableSchema {
    let mut column_schemas =  HashMap::new();
    zeus_table_schema.get_fields()
      .values()
      .map(|f| f.clone())
      .map(|f| ColumnSchema::new(f))
      .for_each(|c| {
        column_schemas.insert(c.get_id(), Arc::new(c));
      });

    TableSchema {
      inner: Arc::new(zeus_table_schema),
      column_schemas
    }
  }

  pub fn get_table_engine(&self) -> String {
    self.inner.get_format().to_string()
  }

  pub fn get_id(&self) -> i32 {
    self.inner.get_id()
  }

  pub fn get_column_schema(
    &self,
    column_id: i32,
  ) -> Option<Arc<ColumnSchema>> {
    self.column_schemas.get(&column_id)
      .map(|s| s.clone())
  }
}
