use std::sync::Arc;
use std::any::Any;

use arrow::array::Array;
use arrow::array::PrimitiveArray;
use arrow::array::BinaryArray;
use arrow::datatypes::DataType;
use arrow::datatypes::ArrowPrimitiveType;

use rpc::zeus_meta::ColumnValue;
use util::errors::*;

pub trait Column {
  fn get_column_value(&self, idx: usize) -> Result<ColumnValue>;
}

pub fn to_column(array: Arc<Array>) -> Result<Arc<Column>> {
  let c = match array.data_type() {
    DataType::Boolean => PrimitiveColumn::<bool>::new(array.clone()),
//    DataType::Int32 => PrimitiveColumn::<i32>::new(array),
//    DataType::Int64 => PrimitiveColumn::<i64>::new(array),
//    DataType::Float32 => PrimitiveColumn::<f32>::new(array),
//    DataType::Float64 => PrimitiveColumn::<f64>::new(array),
    dt => bail!("Unrecoginized data type {:?}", dt)
  };
  bail!("Failed")
}






struct PrimitiveColumn<T: ArrowPrimitiveType + Into<ColumnValue>> {
  array: Arc<PrimitiveArray<T>>
}

macro_rules! def_primitive_column {
  ($native_ty: ident) => {
    impl Column for PrimitiveColumn<$native_ty> {
      fn get_column_value(&self, idx: usize) -> Result<ColumnValue> {
        let idx = idx as i64;
        if idx>=0 && idx<self.array.len() {
          Ok(self.array.value(idx).into())
        } else {
          bail!("{} out of range.", idx)
        }
      }
    }

    impl PrimitiveColumn<$native_ty> {
      pub fn new(array: Arc<dyn Any>) -> Result<Arc<Column>> {
        let arr = array.downcast::<PrimitiveArray<$native_ty>>()
          .map_err(|| format!("Unable to convert type {:?}", array.get_type_id()))?;

        Ok(Arc::new(Self {
          array: arr
        }))

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

struct Utf8Column {
  array: BinaryArray
}

impl Column for Utf8Column {
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