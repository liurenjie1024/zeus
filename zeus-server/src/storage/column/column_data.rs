use std::vec::Vec;
use std::clone::Clone;
use std::convert::From;
use std::convert::Into;

use rpc::zeus_meta::ColumnValue;
use rpc::zeus_meta::ColumnType;
use rpc::zeus_expr::LiteralExpression;

use util::errors::*;

#[derive(Debug, Clone)]
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

impl<'a> From<&'a LiteralExpression> for Datum {
  fn from(rpc_expr: &LiteralExpression) -> Self {
    let column_value = rpc_expr.get_value();
    match rpc_expr.get_field_type() {
      ColumnType::BOOL => Datum::Bool(column_value.get_bool_value()),
      ColumnType::INT8 => Datum::Int8(column_value.get_i32_value() as i8),
      ColumnType::INT16 => Datum::Int16(column_value.get_i32_value() as i16),
      ColumnType::INT32 => Datum::Int32(column_value.get_i32_value()),
      ColumnType::INT64 => Datum::Int64(column_value.get_i64_value()),
      ColumnType::FLOAT4 => Datum::Float4(column_value.get_float_value()),
      ColumnType::FLOAT8 => Datum::Float8(column_value.get_double_value()),
      ColumnType::TIMESTAMP => Datum::Int64(column_value.get_i64_value()),
      ColumnType::STRING => Datum::UTF8(column_value.get_string_value().to_string()),
    }
  }
}

impl Datum {
  pub fn add(left: &Datum, right: &Datum) -> Result<Datum> {
    match (left, right) {
      (&Datum::Int8(v1), &Datum::Int8(v2)) => Ok(Datum::Int8(v1+v2)),
      (&Datum::Int16(v1), &Datum::Int16(v2)) => Ok(Datum::Int16(v1+v2)),
      (&Datum::Int32(v1), &Datum::Int32(v2)) => Ok(Datum::Int32(v1+v2)),
      (&Datum::Int64(v1), &Datum::Int64(v2)) => Ok(Datum::Int64(v1+v2)),
      (&Datum::Float4(v1), &Datum::Float4(v2)) => Ok(Datum::Float4(v1+v2)),
      (&Datum::Float8(v1), &Datum::Float8(v2)) => Ok(Datum::Float8(v1+v2)),
      (left, right) => bail!("{:?} and {:?} can't be added together.", left, right)
    }
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