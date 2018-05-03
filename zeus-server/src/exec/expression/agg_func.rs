use exec::Block;
use storage::column::column_data::Datum;
use util::errors::*;


pub trait AggFunc: Send {
  fn aggregate(&mut self, args: &Block, pos: usize) -> Result<()>;
  fn collect(&mut self) -> Result<Datum>;
}

pub struct Sum {
  result: Option<Datum>
}




