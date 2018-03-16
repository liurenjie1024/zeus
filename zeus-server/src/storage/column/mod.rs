use std::borrow::ToOwned;
use std::boxed::Box;
use std::iter::Iterator;

use rpc::zeus_meta::FieldType;
use rpc::zeus_data::ColumnValue;
use util::cow_ptr::ToBoxedOwned;
use util::error::Result;

pub mod column_string;
pub mod column_vector;

pub type BoolColumn = column_vector::ColumnVector<bool>;
pub type ByteColumn = column_vector::ColumnVector<u8>;
pub type FloatColumn = column_vector::ColumnVector<f32>;
pub type IntColumn = column_vector::ColumnVector<i32>;
pub type LongColumn = column_vector::ColumnVector<i64>;
pub type TimestampColumn = column_vector::ColumnVector<u64>;
pub type StringColumn = column_string::ColumnString;

pub type ColumnValueIter = Box<Iterator<Item = ColumnValue>>;

pub trait Column: ToBoxedOwned + Send + 'static {
  fn size(&self) -> usize;
  fn field_type(&self) -> FieldType;
  fn iter(&self) -> ColumnValueIter;
}

pub trait ColumnFactory: Send + 'static {
  fn create_column(
    &mut self,
    raw_data: &[u8],
  ) -> Result<Box<Column>>;
}
