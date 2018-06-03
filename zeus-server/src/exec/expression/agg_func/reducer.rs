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
  fn aggregate(&mut self, _args: &Block, _pos: usize) -> Result<()> {
    _args.columns_slice().get(0)
      .and_then(|c| c.get(0))
      .ok_or_else(|| ErrorKind::IndexOutOfBound(_pos, _args.len()).into())
      .and_then(|d| self.do_aggregate(&d))
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

  pub fn sum() -> Reducer {
    Reducer {
      id: AggFuncId::SUM,
      result: None,
      aggregator: Datum::add
    }
  }
}
