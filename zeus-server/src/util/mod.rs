pub mod collections;
pub mod cow_ptr;
pub mod errors;

use std::io::Read;

pub struct EmptyRead {}

impl Read for EmptyRead {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
    panic!("This should not be called!");
  }
}