use std::alloc::Layout;

use arrow::array::Array;

pub trait AggregationFunction {
  fn data_layout(&self) -> Layout;
  unsafe fn init_data(&self, buf: *mut u8);
  unsafe fn destroy_data(&self, buf: *mut u8) {}
  unsafe fn aggregate_all(&self, buf: *mut u8, input: &Array);
//  unsafe fn collect<T>(&self, buf: *mut u8) -> T;
}