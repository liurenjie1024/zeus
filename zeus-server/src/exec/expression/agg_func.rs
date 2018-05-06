use exec::Block;
use storage::column::column_data::Datum;
use util::errors::*;


pub trait AggFunc: Send {
  fn aggregate(&mut self, args: &Block, pos: usize) -> Result<()>;
  fn collect(&mut self) -> Result<Datum>;
}

pub struct Reducer {
  result: Option<Datum>,
  _aggregator: fn(&Datum, &Datum) -> Result<Datum>
}

impl AggFunc for Reducer {
  fn aggregate(&mut self, _args: &Block, _pos: usize) -> Result<()> {
    unimplemented!()
  }
  fn collect(&mut self) -> Result<Datum> {
    match &self.result {
      &Some(ref v) => Ok(v.clone()),
      &None => bail!("Data is empty, this should not happen.")
    }
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
      result: None,
      _aggregator: Datum::add
    }
  }
}

