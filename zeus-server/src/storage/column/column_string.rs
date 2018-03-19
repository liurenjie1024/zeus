use std::vec::Vec;
use std::any::Any;
use std::sync::Arc;

use rpc::zeus_meta::FieldType;
use storage::column::Column;
use storage::column::ColumnValueIter;
use util::error::Result;
use util::cow_ptr::ToBoxedOwned;

pub struct ColumnString {
  offsets: Arc<Vec<usize>>,
  chars: Arc<Vec<u8>>,
}

impl Column for ColumnString {
  fn size(&self) -> usize {
    self.offsets.len()
  }

  fn field_type(&self) -> FieldType {
    FieldType::STRING
  }
  fn into_iter(&self) -> ColumnValueIter {
    unimplemented!()
  }
}

impl ToBoxedOwned for ColumnString {
  fn to_boxed_owned(&self) -> Box<Any> {
    Box::new(ColumnString {
      offsets: self.offsets.clone(),
      chars: self.chars.clone(),
    })
  }
}

impl ColumnString {
  pub fn new(
    offsets: Vec<usize>,
    chars: Vec<u8>,
  ) -> Result<ColumnString>
  {
    Ok(ColumnString {
      offsets: Arc::new(offsets),
      chars: Arc::new(chars),
    })
  }
}