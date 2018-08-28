use std::convert::TryFrom;

use arrow::array::Array;
use arrow::array::PrimitiveArray;
use arrow::array::BinaryArray;
use arrow::datatypes::DataType;
use arrow::datatypes::ArrowPrimitiveType;

use rpc::zeus_meta::ColumnValue;
use util::errors::*;
pub enum Column<'a> {
  Bool(PrimitiveColumn<'a, bool>),
  Int32(PrimitiveColumn<'a, i32>),
  Int64(PrimitiveColumn<'a, i64>),
  Float(PrimitiveColumn<'a, f32>),
  Double(PrimitiveColumn<'a, f64>),
  Utf8(Utf8Column<'a>)
}

impl<'a> Column<'a> {
  pub fn get_column_value(&self, idx: usize) -> Result<ColumnValue> {
    match self {
      Column::Bool(inner) => inner.get_column_value(idx),
      Column::Int32(inner) => inner.get_column_value(idx),
      Column::Int64(inner) => inner.get_column_value(idx),
      Column::Float(inner) => inner.get_column_value(idx),
      Column::Double(inner) => inner.get_column_value(idx),
      Column::Utf8(inner) => inner.get_column_value(idx)
    }
  }
}


pub fn to_column(array: &Array) -> Result<Column> {
  let c = match array.data_type() {
    DataType::Boolean => Column::Bool(PrimitiveColumn::try_from(array)?),
    DataType::Int32 => Column::Int32(PrimitiveColumn::try_from(array)?),
    DataType::Int64 => Column::Int64(PrimitiveColumn::try_from(array)?),
    DataType::Float32 => Column::Float(PrimitiveColumn::try_from(array)?),
    DataType::Float64 => Column::Double(PrimitiveColumn::try_from(array)?),
    DataType::Utf8 => Column::Utf8(Utf8Column::try_from(array)?),
    dt => bail!("Unrecoginized data type {:?}", dt)
  };
  Ok(c)
}

pub struct PrimitiveColumn<'a, T>
  where T: ArrowPrimitiveType
{
  array: &'a PrimitiveArray<T>
}

macro_rules! def_primitive_column {
  ($native_ty: ident) => {
    impl<'a> PrimitiveColumn<'a, $native_ty> {
      fn get_column_value(&self, idx: usize) -> Result<ColumnValue> {
        let idx = idx as i64;
        if idx>=0 && idx<self.array.len() {
          Ok(self.array.value(idx).into())
        } else {
          bail!("{} out of range.", idx)
        }
      }
    }

    impl<'a> TryFrom<&'a Array> for PrimitiveColumn<'a, $native_ty> {
      type Error = Error;
      fn try_from(array: &'a Array) -> Result<Self> {
        let data_type = array.data_type();
        array.as_any().downcast_ref::<PrimitiveArray<$native_ty>>()
          .ok_or_else(|| format!("Unable to convert data type {:?}", data_type).into())
          .map(|array| Self {
            array
          })
      }
    }
  };
}

def_primitive_column!(bool);
def_primitive_column!(i32);
def_primitive_column!(i64);
def_primitive_column!(f32);
def_primitive_column!(f64);

impl Into<ColumnValue> for bool {
  fn into(self) -> ColumnValue {
    let mut cv = ColumnValue::new();
    cv.set_bool_value(self);
    cv
  }
}

impl Into<ColumnValue> for i32 {
  fn into(self) -> ColumnValue {
    let mut cv = ColumnValue::new();
    cv.set_i32_value(self);
    cv
  }
}

impl Into<ColumnValue> for i64 {
  fn into(self) -> ColumnValue {
    let mut cv = ColumnValue::new();
    cv.set_i64_value(self);
    cv
  }
}

impl Into<ColumnValue> for f32 {
  fn into(self) -> ColumnValue {
    let mut cv = ColumnValue::new();
    cv.set_float_value(self);
    cv
  }
}

impl Into<ColumnValue> for f64 {
  fn into(self) -> ColumnValue {
    let mut cv = ColumnValue::new();
    cv.set_double_value(self);
    cv
  }
}

pub struct Utf8Column<'a> {
  array: &'a BinaryArray
}

impl<'a> Utf8Column<'a> {
  fn get_column_value(&self, idx: usize) -> Result<ColumnValue> {
    let idx = idx as i64;
    if idx>=0 && idx<self.array.len() {
      let mut cv = ColumnValue::new();
      cv.set_string_value(self.array.get_string(idx));
      Ok(cv)
    } else {
      bail!("{} out of range.", idx)
    }
  }
}

impl<'a> TryFrom<&'a Array> for Utf8Column<'a> {
  type Error = Error;
  fn try_from(array: &'a Array) -> Result<Self> {
    array.as_any()
      .downcast_ref::<BinaryArray>()
      .ok_or_else(|| "Only binary array can convert to utf8 column".into())
      .map(|array| Self {
        array
      })
  }
}
