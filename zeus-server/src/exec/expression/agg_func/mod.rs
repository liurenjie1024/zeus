mod reducer;
mod count;

use exec::Block;
use storage::column::vec_column_data::Datum;
use rpc::zeus_expr::AggFuncId;
use util::errors::*;

pub trait AggFunc: Send {
  fn aggregate(&mut self, args: &Block, pos: usize) -> Result<()>;
  fn aggregate_all(&mut self, args: &Block) -> Result<()>;
  fn collect(&mut self) -> Result<Datum>;
}

pub fn func_of(id: AggFuncId) -> Box<AggFunc> {
  match id {
    AggFuncId::SUM_INT32 => box reducer::Reducer::sum(id),
    AggFuncId::SUM_INT64 => box reducer::Reducer::sum(id),
    AggFuncId::SUM_FLOAT4 => box reducer::Reducer::sum(id),
    AggFuncId::SUM_FLOAT8 => box reducer::Reducer::sum(id),

    AggFuncId::MAX_INT32 => box reducer::Reducer::max(id),
    AggFuncId::MAX_INT64 => box reducer::Reducer::max(id),
    AggFuncId::MAX_FLOAT4 => box reducer::Reducer::max(id),
    AggFuncId::MAX_FLOAT8 => box reducer::Reducer::max(id),
    AggFuncId::MAX_STR => box reducer::Reducer::max(id),

    AggFuncId::MIN_INT32 => box reducer::Reducer::min(id),
    AggFuncId::MIN_INT64 => box reducer::Reducer::min(id),
    AggFuncId::MIN_FLOAT4 => box reducer::Reducer::min(id),
    AggFuncId::MIN_FLOAT8 => box reducer::Reducer::min(id),
    AggFuncId::MIN_STR => box reducer::Reducer::min(id),

    AggFuncId::COUNT => box count::Counter::new()
  }
}



