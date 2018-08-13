use std::iter::Iterator;

use exec::Block;
use storage::column::vec_column_data::Datum;
use super::AggFunc;
use rpc::zeus_expr::AggFuncId;
use util::errors::*;

pub(super) struct Reducer {
  id: AggFuncId,
  result: Option<Datum>,
  aggregator: fn(&Datum, &Datum) -> Result<Datum>
}

impl AggFunc for Reducer {

  fn aggregate(&mut self, args: &Block, pos: usize) -> Result<()> {
    args.columns_slice().get(0)
      .and_then(|c| c.get(pos))
      .ok_or_else(|| ErrorKind::IndexOutOfBound(pos, args.len()).into())
      .and_then(|d| self.do_aggregate(&d))
  }

  fn aggregate_all(&mut self, args: &Block) -> Result<()> {
    (0..args.len())
      .try_for_each(|idx| self.aggregate(args, idx))
  }

  fn collect(&mut self) -> Result<Datum> {
    self.result.take()
      .ok_or_else(|| ErrorKind::EmptyAggregator(self.id).into())
  }
}

impl Reducer {
  fn do_aggregate(&mut self, data: &Datum) -> Result<()> {
    let result =  match &self.result {
      &Some(ref v) => (self.aggregator)(v, data),
      &None => Ok(data.clone())
    }?;

    self.result = Some(result);
    Ok(())
  }

  pub fn sum(agg_func_id: AggFuncId) -> Reducer {
    Reducer {
      id: agg_func_id,
      result: None,
      aggregator: Datum::add
    }
  }

  pub fn min(agg_func_id: AggFuncId) -> Reducer {
    Reducer {
      id: agg_func_id,
      result: None,
      aggregator: Datum::min
    }
  }

  pub fn max(agg_func_id: AggFuncId) -> Reducer {
    Reducer {
      id: agg_func_id,
      result: None,
      aggregator: Datum::max
    }
  }
}
