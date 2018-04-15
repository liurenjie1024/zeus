pub mod column_string;
pub mod column_vector;
pub mod column_data;
mod field;

use std::boxed::Box;
use std::iter::Iterator;
use std::slice::Iter;

use self::column_data::ColumnData;
use self::column_data::Datum;
use rpc::zeus_meta::ColumnType;
use rpc::zeus_data::ColumnValue;
use util::cow_ptr::ToBoxedOwned;
use util::errors::*;


pub type BoolColumn = column_vector::ColumnVector<bool>;
pub type ByteColumn = column_vector::ColumnVector<u8>;
pub type FloatColumn = column_vector::ColumnVector<f32>;
pub type IntColumn = column_vector::ColumnVector<i32>;
pub type LongColumn = column_vector::ColumnVector<i64>;
pub type TimestampColumn = column_vector::ColumnVector<u64>;
pub type StringColumn = column_string::ColumnString;

// pub type ColumnValueIter = Box<Iterator<Item = ColumnValue>>;

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
