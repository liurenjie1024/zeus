use std::convert::Into;

use rpc::zeus_data::ColumnValue;

impl Into<ColumnValue> for bool {
  fn into(self) -> ColumnValue {
    let mut v = ColumnValue::new();
    v.set_bool_value(self);
    v
  }
}

impl Into<ColumnValue> for u8 {
  fn into(self) -> ColumnValue {
    let mut v = ColumnValue::new();
    v.set_i32_value(self as i32);
    v
  }
}


impl Into<ColumnValue> for f32 {
  fn into(self) -> ColumnValue {
    let mut v = ColumnValue::new();
    v.set_float_value(self);
    v
  }
}

impl Into<ColumnValue> for i32 {
  fn into(self) -> ColumnValue {
    let mut v = ColumnValue::new();
    v.set_i32_value(self);
    v
  }
}

impl Into<ColumnValue> for i64 {
  fn into(self) -> ColumnValue {
    let mut v = ColumnValue::new();
    v.set_i64_value(self);
    v
  }
}

impl Into<ColumnValue> for u64 {
  fn into(self) -> ColumnValue {
    let mut v = ColumnValue::new();
    v.set_i64_value(self as i64);
    v
  }
}
