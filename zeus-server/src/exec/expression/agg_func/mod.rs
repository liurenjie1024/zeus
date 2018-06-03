mod reducer;

use exec::Block;
use storage::column::vec_column_data::Datum;
use rpc::zeus_expr::AggFuncId;
use util::errors::*;

pub trait AggFunc: Send {
  fn aggregate(&mut self, args: &Block, pos: usize) -> Result<()>;
  fn collect(&mut self) -> Result<Datum>;
}

pub fn func_of(id: AggFuncId) -> Box<AggFunc> {
  match id {
    AggFuncId::SUM => box reducer::Reducer::sum()
  }
}



