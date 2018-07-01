mod reducer;
mod count;

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
    AggFuncId::SUM_I32 => box reducer::Reducer::sum(id),
    AggFuncId::SUM_I64 => box reducer::Reducer::sum(id),
    AggFuncId::SUM_F4 => box reducer::Reducer::sum(id),
    AggFuncId::SUM_F8 => box reducer::Reducer::sum(id),

    AggFuncId::MAX_I32 => box reducer::Reducer::max(id),
    AggFuncId::MAX_I64 => box reducer::Reducer::max(id),
    AggFuncId::MAX_F4 => box reducer::Reducer::max(id),
    AggFuncId::MAX_F8 => box reducer::Reducer::max(id),
    AggFuncId::MAX_STR => box reducer::Reducer::max(id),

    AggFuncId::MIN_I32 => box reducer::Reducer::min(id),
    AggFuncId::MIN_I64 => box reducer::Reducer::min(id),
    AggFuncId::MIN_F4 => box reducer::Reducer::min(id),
    AggFuncId::MIN_F8 => box reducer::Reducer::min(id),
    AggFuncId::MIN_STR => box reducer::Reducer::min(id),

    AggFuncId::COUNT => box count::Counter::new()
  }
}



