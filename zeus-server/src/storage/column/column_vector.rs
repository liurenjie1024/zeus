use std::vec::Vec;
use std::boxed::Box;
use std::iter::Iterator;
use std::sync::Arc;
use std::any::Any;
use std::convert::Into;

use super::Column;
use rpc::zeus_meta::FieldType;
use rpc::zeus_data::ColumnValue;
use util::cow_ptr::ToBoxedOwned;
use util::error::Result;

pub struct ColumnVector<T>
where T: Send + Sync + Copy + Into<ColumnValue> + 'static
{
  field_type: FieldType,
  data: Arc<Vec<T>>,
}

struct ColumnVectorIterator<T>
where T: Copy + Into<ColumnValue>
{
  cur_pos: usize,
  column_vec: Arc<Vec<T>>,
}

impl<T> Iterator for ColumnVectorIterator<T>
where T: Copy + Into<ColumnValue>
{
  type Item = ColumnValue;

  fn next(&mut self) -> Option<Self::Item> {
    let v = self.column_vec.get(self.cur_pos).map(|&x| x.into());
    self.cur_pos += 1;
    v
  }
}

impl<T> Column for ColumnVector<T>
where T: Send + Sync + Copy + Into<ColumnValue> + 'static
{
  fn size(&self) -> usize {
    self.data.len()
  }

  fn field_type(&self) -> FieldType {
    self.field_type
  }
  fn into_iter(&self) -> Box<Iterator<Item = ColumnValue>> {
    box ColumnVectorIterator {
      cur_pos: 0usize,
      column_vec: self.data.clone(),
    }
  }
}

impl<T> ToBoxedOwned for ColumnVector<T>
where T: Send + Sync + Copy + Into<ColumnValue> + 'static
{
  fn to_boxed_owned(&self) -> Box<Any> {
    Box::new(ColumnVector {
      field_type: self.field_type,
      data: self.data.clone(),
    })
  }
}

impl<T> ColumnVector<T>
where T: Send + Sync + Copy + Into<ColumnValue> + 'static
{
  pub fn create(
    field_type: FieldType,
    data: Vec<T>,
  ) -> Result<ColumnVector<T>>
  {
    Ok(ColumnVector {
      field_type,
      data: Arc::new(data),
    })
  }
}
