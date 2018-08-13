
use exec::Block;
use storage::column::vec_column_data::Datum;
use super::AggFunc;

use util::errors::*;

pub struct Counter {
  result: i32
}

impl AggFunc for Counter {
  fn aggregate(&mut self, _args: &Block, _pos: usize) -> Result<()> {
    self.result += 1;
    Ok(())
  }

  fn aggregate_all(&mut self, args: &Block) -> Result<()> {
    self.result += args.len() as i32;
    Ok(())
  }

  fn collect(&mut self) -> Result<Datum> {
    let ret = Ok(Datum::Int32(self.result));
    self.result = 0;
    ret
  }
}

impl Counter {
  pub fn new() -> Counter {
    Counter {
      result: 0
    }
  }
}
