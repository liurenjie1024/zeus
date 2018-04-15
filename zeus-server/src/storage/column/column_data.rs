use std::vec::Vec;
use std::clone::Clone;
use std::convert::From;
use std::convert::Into;

use rpc::zeus_data::ColumnValue;

pub enum Datum {
  Bool(bool),
  Int8(i8),
  Int16(i16),
  Int32(i32),
  Int64(i64),
  Float4(f32),
  Float8(f64),
  UTF8(String)
}

impl<'a> Into<ColumnValue> for &'a Datum {
  fn into(self) -> ColumnValue {
    let mut column_value = ColumnValue::new();
    match *self {
      Datum::Bool(v) => column_value.set_bool_value(v),
      Datum::Int8(v) => column_value.set_i32_value(v as i32),
      Datum::Int16(v) => column_value.set_i32_value(v as i32),
      Datum::Int32(v) => column_value.set_i32_value(v),
      Datum::Int64(v) => column_value.set_i64_value(v),
      Datum::Float4(v) => column_value.set_float_value(v),
      Datum::Float8(v) => column_value.set_double_value(v),
      Datum::UTF8(ref v) => column_value.set_string_value(v.clone())
    }
    column_value
  }
}

macro_rules! datum_from {
  ($dt: ty, $var: ident) => {
    impl From<$dt> for Datum {
      fn from(t: $dt) -> Datum {
        Datum::$var(t)
      }
    }
  };
}

datum_from!(bool, Bool);
datum_from!(i8, Int8);
datum_from!(i16, Int16);
datum_from!(i32, Int32);
datum_from!(i64, Int64);
datum_from!(f32, Float4);
datum_from!(f64, Float8);
datum_from!(String, UTF8);


pub struct ColumnData {
  pub(super) datums: Vec<Datum>
}

impl ColumnData {
  pub fn len(&self) -> usize {
    self.datums.len()
  }
}

macro_rules! column_data_from_vec {
  ($dt: ty) => {
    impl From<Vec<$dt>> for ColumnData {
      fn from(data: Vec<$dt>) -> ColumnData {
        let datums = data.into_iter()
          .map(|x| x.into())
          .collect::<Vec<Datum>>();
        ColumnData::from(datums)
      }
    }
  };
}

column_data_from_vec!(bool);
column_data_from_vec!(i8);
column_data_from_vec!(i16);
column_data_from_vec!(i32);
column_data_from_vec!(i64);
column_data_from_vec!(f32);
column_data_from_vec!(f64);
column_data_from_vec!(String);

impl From<Vec<Datum>> for ColumnData {
  fn from(datums: Vec<Datum>) -> ColumnData {
    ColumnData {
      datums
    }
  }
}