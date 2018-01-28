use std::collections::HashMap;

pub enum ValueType {
    String,
    Long,
    Float
}

pub struct DBSchema {
    pub id: i32,
    pub tables: HashMap<i32, TableSchema>
}

pub struct TableSchema {
    pub id: i32,
    pub fields: HashMap<i32, FieldInfo>
}

pub struct FieldInfo {
    pub id: i32,
    pub value_type: ValueType,
    pub is_metric: bool
}

