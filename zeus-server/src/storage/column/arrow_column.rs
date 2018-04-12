use std::any::Any;

use rpc::zeus_meta::ColumnType;
use util::cow_ptr::ToBoxedOwned;
use super::ColumnValueIter;
use super::Column;

pub struct ArrowColumn {}

impl Column for ArrowColumn {
  fn size(&self) -> usize {
    unimplemented!()
  }

  fn field_type(&self) -> ColumnType {
    unimplemented!()
  }

  fn into_iter(&self) -> ColumnValueIter {
    unimplemented!()
  }
}

impl ToBoxedOwned for ArrowColumn {
  fn to_boxed_owned(&self) -> Box<Any> {
    unimplemented!()
  }
}
