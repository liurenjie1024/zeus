use rpc::zeus_meta::ColumnType;

pub trait FieldTypeInfo {
  fn field_size(&self) -> Option<usize>;
  fn is_fixed_size(&self) -> bool;
}

impl FieldTypeInfo for ColumnType {
  fn field_size(&self) -> Option<usize> {
    match *self {
      ColumnType::STRING => None,
      ColumnType::BOOL => Some(1),
      ColumnType::BYTE => Some(1),
      ColumnType::FLOAT => Some(4),
      ColumnType::INT32 => Some(4),
      ColumnType::INT64 => Some(8),
      ColumnType::TIMESTAMP => Some(8),
    }
  }

  fn is_fixed_size(&self) -> bool {
    match *self {
      ColumnType::STRING => false,
      ColumnType::BOOL => true,
      ColumnType::BYTE => true,
      ColumnType::FLOAT => true,
      ColumnType::INT32 => true,
      ColumnType::INT64 => true,
      ColumnType::TIMESTAMP => true,
    }
  }
}
