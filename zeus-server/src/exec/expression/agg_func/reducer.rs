use exec::Block;
use storage::column::vec_column_data::Datum;
use super::AggFunc;
use rpc::zeus_expr::AggFuncId;
use util::errors::*;

pub(super) struct Reducer {
  id: AggFuncId,
  result: Option<Datum>,
  _aggregator: fn(&Datum, &Datum) -> Result<Datum>
}

impl AggFunc for Reducer {
  fn aggregate(&mut self, _args: &Block, _pos: usize) -> Result<()> {
    unimplemented!()
  }
  fn collect(&mut self) -> Result<Datum> {
    self.result.take()
      .ok_or_else(|| ErrorKind::EmptyAggregator(self.id).into())
  }
}

impl Reducer {
  fn _do_aggregate(&mut self, data: &Datum) -> Result<()> {
    let result =  match &self.result {
      &Some(ref v) => (self._aggregator)(v, data),
      &None => Ok(data.clone())
    }?;

    self.result = Some(result);
    Ok(())
  }

  pub fn sum() -> Reducer {
    Reducer {
      id: AggFuncId::SUM,
      result: None,
      _aggregator: Datum::add
    }
  }
}
