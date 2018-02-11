use std::borrow::ToOwned;
use std::boxed::Box;

use rpc::zeus_meta::FieldType;
use util::cow_ptr::ToBoxedOwned;

pub mod column_vector;

pub type BoolColumn = column_vector::ColumnVector<bool>;
pub type FloatColumn = column_vector::ColumnVector<f32>;
pub type IntColumn = column_vector::ColumnVector<i32>;
pub type LongColumn = column_vector::ColumnVector<i64>;
pub type TimestampColumn = column_vector::ColumnVector<u64>;

pub trait Column: ToBoxedOwned {
    fn size(&self) -> usize;
    fn field_type(&self) -> FieldType;
}




