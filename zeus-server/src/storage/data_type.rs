use rpc::zeus_meta::FieldType;

pub trait FieldTypeInfo {
  fn field_size(&self) -> Option<usize>;
  fn is_fixed_size(&self) -> bool;
}

impl FieldTypeInfo for FieldType {
  fn field_size(&self) -> Option<usize> {
    match *self {
      FieldType::STRING => None,
      FieldType::BOOL => Some(1),
      FieldType::BYTE => Some(1),
      FieldType::FLOAT => Some(4),
      FieldType::INT32 => Some(4),
      FieldType::INT64 => Some(8),
      FieldType::TIMESTAMP => Some(8),
    }
  }

  fn is_fixed_size(&self) -> bool {
    match *self {
      FieldType::STRING => false,
      FieldType::BOOL => true,
      FieldType::BYTE => true,
      FieldType::FLOAT => true,
      FieldType::INT32 => true,
      FieldType::INT64 => true,
      FieldType::TIMESTAMP => true,
    }
  }
}
