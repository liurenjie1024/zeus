use std::alloc::Layout;

use arrow::array::Array;

use exec2::column::ColumnBuilder;

pub trait AggregationFunction {
  fn name(&self) -> &str;
  fn data_layout(&self) -> Layout;
  unsafe fn init_data(&self, buf: *mut u8);
  unsafe fn destroy_data(&self, buf: *mut u8) {}
  unsafe fn aggregate_all(&self, buf: *mut u8, input: &Array);
  unsafe fn collect(&mut self, buf: *mut u8, column: &mut ColumnBuilder);
}