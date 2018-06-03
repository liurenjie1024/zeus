use std::vec::Vec;
use std::clone::Clone;
use std::convert::From;
use std::convert::Into;
use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem::transmute;

use rpc::zeus_meta::ColumnValue;
use rpc::zeus_meta::ColumnType;
use rpc::zeus_expr::LiteralExpression;
use super::ColumnData;
use super::ColumnIter;

use util::errors::*;

#[derive(Debug, Clone, PartialEq)]
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

impl Datum {
  pub fn new_literal_expr(rpc_expr: &LiteralExpression, field_type: ColumnType) -> Self {
    let column_value = rpc_expr.get_value();
    match field_type {
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
  pub fn vec_of<T>(vec_data: Vec<T>) -> Vec<Datum>
    where T: Into<Datum> + Clone {
    vec_data.iter()
      .map(|x| x.clone().into())
      .collect()
  }

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

  pub fn try_cmp(left: &Datum, right: &Datum) -> Result<Ordering> {
    match (left, right) {
      (&Datum::Int8(v1), &Datum::Int8(v2)) => Ok(v1.cmp(&v2)),
      (&Datum::Int16(v1), &Datum::Int16(v2)) => Ok(v1.cmp(&v2)),
      (&Datum::Int32(v1), &Datum::Int32(v2)) => Ok(v1.cmp(&v2)),
      (&Datum::Int64(v1), &Datum::Int64(v2)) => Ok(v1.cmp(&v2)),
      (&Datum::Float4(v1), &Datum::Float4(v2)) => {
        let msg = format!("Unable to compare: '{}', '{}'", v1, v2);
        v1.partial_cmp(&v2).ok_or(ErrorKind::UnableToCompare(msg).into())
      }
      (&Datum::Float8(v1), &Datum::Float8(v2)) => {
        let msg = format!("Unable to compare: '{}', '{}'", v1, v2);
        v1.partial_cmp(&v2).ok_or(ErrorKind::UnableToCompare(msg).into())
      },
      (&Datum::UTF8(ref v1), &Datum::UTF8(ref v2)) => Ok(v1.cmp(&v2)),
      (left, right) => bail!("{:?} and {:?} can't be added together.", left, right)
    }
  }

  pub fn and(left: &Datum, right: &Datum) -> Result<Datum> {
    match (left, right) {
      (&Datum::Bool(v1), &Datum::Bool(v2)) => Ok(Datum::Bool(v1 & v2)),
      (_, _) => bail!("Logical operator and can only be applied to booleans")
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


impl Datum {
  pub fn to_bool(&self) -> Result<bool> {
    match self {
      Datum::Bool(v) => Ok(*v),
      _ => bail!("'{:?}' can't cast to bool.", self)
    }
  }

  pub fn to_i8(&self) -> Result<i8> {
    match self {
      Datum::Int8(v) => Ok(*v),
      _ => bail!("'{:?}' can't cast to i8.", self)
    }
  }

  pub fn to_i16(&self) -> Result<i16> {
    match self {
      Datum::Int8(v) => Ok(*v as i16),
      Datum::Int16(v) => Ok(*v),
      _ => bail!("'{:?}' can't cast to i16.", self)
    }
  }

  pub fn to_i32(&self) -> Result<i32> {
    match self {
      Datum::Int8(v)  => Ok(*v as i32),
      Datum::Int16(v)  => Ok(*v as i32),
      Datum::Int32(v) => Ok(*v),
      _ => bail!("'{:?}' can't cast to i32.", self)
    }
  }

  pub fn to_i64(&self) -> Result<i64> {
    match self {
      Datum::Int8(v) => Ok(*v as i64),
      Datum::Int16(v) => Ok(*v as i64),
      Datum::Int32(v) => Ok(*v as i64),
      Datum::Int64(v) => Ok(*v as i64),
      _ => bail!("'{:?}' can't cast to i64.", self)
    }
  }

  pub fn to_f32(&self) -> Result<f32> {
    match self {
      Datum::Int8(v) => Ok(*v as f32),
      Datum::Int16(v) => Ok(*v as f32),
      Datum::Int32(v) => Ok(*v as f32),
      Datum::Int64(v) => Ok(*v as f32),
      Datum::Float4(v) => Ok(*v),
      _ => bail!("'{:?}' can't cast to f32.", self)
    }
  }

  pub fn to_f64(&self) -> Result<f64> {
    match self {
      Datum::Int8(v)  => Ok(*v as f64),
      Datum::Int16(v)  => Ok(*v as f64),
      Datum::Int32(v)  => Ok(*v as f64),
      Datum::Int64(v)  => Ok(*v as f64),
      Datum::Float4(v)  => Ok(*v as f64),
      Datum::Float8(v) => Ok(*v),
      _ => bail!("'{:?}' can't cast to f64.", self)
    }
  }

  pub fn to_str(&self) -> Result<&str> {
    match self {
      Datum::UTF8(ref v) => Ok(v.as_str()),
      _ => bail!("'{:?}' can't cast to str.", self)
    }
  }
}

impl Eq for Datum {}

impl Hash for Datum {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Datum::Bool(v) => v.hash(state),
      Datum::Int8(v) => v.hash(state),
      Datum::Int16(v) => v.hash(state),
      Datum::Int32(v) => v.hash(state),
      Datum::Int64(v) => v.hash(state),
      Datum::Float4(v) => unsafe {
        state.write(&transmute::<f32, [u8; 4]>(*v))
      }
      Datum::Float8(v) => unsafe {
        state.write(&transmute::<f64, [u8; 8]>(*v))
      }
      Datum::UTF8(v) => v.hash(state)
    }
  }
}

#[derive(Clone, Debug)]
pub struct VecColumnData {
  pub(super) datums: Vec<Datum>
}

impl VecColumnData {
  pub fn len(&self) -> usize {
    self.datums.len()
  }
}

impl ColumnData for VecColumnData {
  fn len(&self) -> usize  {
    self.datums.len()
  }

  fn get(&self, idx: usize) -> Option<Datum> {
    self.datums.get(idx)
      .map(|x| x.clone())
  }

  fn iter(&self) -> ColumnIter {
    ColumnIter {
      column_data: self,
      idx: 0 as usize
    }
  }
}

macro_rules! column_data_from_vec {
  ($dt: ty) => {
    impl From<Vec<$dt>> for VecColumnData {
      fn from(data: Vec<$dt>) -> VecColumnData {
        let datums = data.into_iter()
          .map(|x| x.into())
          .collect::<Vec<Datum>>();
        VecColumnData::from(datums)
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

impl From<Vec<Datum>> for VecColumnData {
  fn from(datums: Vec<Datum>) -> VecColumnData {
    VecColumnData {
      datums
    }
  }
}