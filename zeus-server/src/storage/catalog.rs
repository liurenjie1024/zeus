use rpc::zeus_meta::FieldType;

use std::sync::Arc;

pub trait ColumnSchema {
    fn get_id(&self) -> i32;
    fn get_name(&self) -> String;
    fn get_type(&self) -> FieldType;
}

pub trait TableSchema {
    fn get_id(&self) -> i32;
    fn get_column_schema(&self, column_id: i32) -> Option<Arc<ColumnSchema>>;
}


pub trait CatalogManager:  Send + Sync {
    fn get_table_schema(&self, db_id: i32, table_id:i32) -> Option<Arc<TableSchema>>;
}
