use std::vec::Vec;
use std::borrow::ToOwned;

use rpc::zeus_meta::FieldType;

pub mod column_vector;

pub type BoolColumn = column_vector::ColumnVector<bool>;
pub type FloatColumn = column_vector::ColumnVector<f32>;
pub type IntColumn = column_vector::ColumnVector<i32>;
pub type LongColumn = column_vector::ColumnVector<i64>;
pub type TimestampColumn = column_vector::ColumnVector<u64>;

pub trait Column: ToOwned {
    fn size(&self) -> usize;
    fn field_type(&self) -> FieldType;
}




