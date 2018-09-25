use std::sync::Arc;
use std::mem::transmute;
use std::mem::size_of;

use arrow::array::Array;
use arrow::array::PrimitiveArray;
use arrow::builder::Builder;
use arrow::datatypes::ArrowPrimitiveType;
use arrow::datatypes::DataType;
use arrow::array_data::ArrayDataBuilder;

use util::errors::*;

pub trait ColumnBuilder {
  unsafe fn append(&mut self, data: *const u8, length: usize) -> Result<()>;
  unsafe fn into_array(self) -> Arc<Array>;
}

struct PrimitiveColumnBuffer<T>
  where T: ArrowPrimitiveType
{
  builder: Builder<T>,
  data_type: DataType
}

impl<T> ColumnBuilder for PrimitiveColumnBuffer<T>
  where T: ArrowPrimitiveType
{
  unsafe fn append(&mut self, data: *const u8, length: usize) -> Result<()> {
    ensure!(length == size_of::<T>(), "Input size is {}, expected size is {}", length,
     size_of::<T>());
    let p = transmute::<*const u8, *const T>(data);
    self.builder.push(*p);
    Ok(())
  }

  unsafe fn into_array(mut self) -> Arc<Array> {
    let buffer = self.builder.finish();
    let array_data = ArrayDataBuilder::new(self.data_type)
      .len(buffer.len() as i64)
      .add_buffer(buffer)
      .build();

    Arc::new(PrimitiveArray::<T>::from(array_data))
  }
}

impl<T> PrimitiveColumnBuffer<T>
  where T: ArrowPrimitiveType
{
  pub fn new(data_type: DataType) -> Self {
    Self {
      builder: Builder::<T>::new(),
      data_type
    }
  }

  pub fn with_capacity(data_type: DataType, capacity: usize) -> Self {
    Self {
      builder: Builder::<T>::with_capacity(capacity),
      data_type
    }
  }
}

#[cfg(test)]
mod tests {
  use std::mem::transmute;

  use arrow::array::Array;
  use arrow::datatypes::DataType;
  use arrow::array::PrimitiveArray;

  use super::ColumnBuilder;
  use super::PrimitiveColumnBuffer;

  #[test]
  fn test_i64_column_builder() {
    let mut builder = PrimitiveColumnBuffer::<i64>::new(DataType::Int64);

    let (v1, v2, v3) = (8i64, 9i64, 10i64);
    unsafe {
      builder.append(transmute::<*const i64, *const u8>(&v1), 8);
      builder.append(transmute::<*const i64, *const u8>(&v2), 8);
      builder.append(transmute::<*const i64, *const u8>(&v3), 8);

      let array = builder.into_array();

      let array_con = array.as_any().downcast_ref::<PrimitiveArray<i64>>().unwrap();
      assert_eq!(3, array_con.len());
      assert_eq!(8i64, array_con.value(0));
      assert_eq!(9i64, array_con.value(1));
      assert_eq!(10i64, array_con.value(2));
    }
  }
}



