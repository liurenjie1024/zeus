pub mod column_data;

use std::iter::Iterator;
use std::slice::Iter;
use std::convert::From;

use self::column_data::ColumnData;
use self::column_data::Datum;
use rpc::zeus_meta::ColumnType;
use rpc::zeus_meta::ColumnValue;
use util::errors::*;


pub struct Column {
  field_type: ColumnType,
  data: ColumnData
}

pub struct ColumnValueIter<'a> {
  data: Iter<'a, Datum>
}

impl Column {
  pub fn new(field_type: ColumnType, data: ColumnData) -> Column {
    Column {
      field_type,
      data
    }
  }

  pub fn size(&self) -> usize {
    self.data.len()
  }

  pub fn field_type(&self) -> ColumnType {
    self.field_type
  }

  pub fn iter(&self) -> ColumnValueIter {
    ColumnValueIter {
      data: self.data.datums.iter()
    }
  }

  pub fn take(&self, num: usize) -> Column {
    let data = self.data.datums.iter()
      .take(num)
      .map(|x| x.clone())
      .collect::<Vec<Datum>>();

    Column {
      field_type: self.field_type,
      data: ColumnData::from(data)
    }
  }
}

impl<'a> Iterator for ColumnValueIter<'a> {
  type Item = ColumnValue;
  fn next(&mut self) -> Option<ColumnValue> {
    self.data.next()
      .map(|x| x.into())
  }
}

pub trait ColumnFactory: Send + 'static {
  fn create_column(
    &mut self,
    raw_data: &[u8],
  ) -> Result<Column>;
}
