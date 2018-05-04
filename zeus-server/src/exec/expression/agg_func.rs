use exec::Block;
use storage::column::column_data::Datum;
use util::errors::*;


pub trait AggFunc: Send {
  fn aggregate(&mut self, args: &Block, pos: usize) -> Result<()>;
  fn collect(&mut self) -> Result<Datum>;
}

pub struct Reducer {
  result: Option<Datum>,
  aggregator: fn(&Datum, &Datum) -> Result<Datum>
}

impl AggFunc for Reducer {
  fn aggregate(&mut self, args: &Block, pos: usize) -> Result<()> {
    unimplemented!()
  }
  fn collect(&mut self) -> Result<Datum> {
    match self.result {
      Some(v) => Ok(v),
      None => bail!("Data is empty, this should not happen.")
    }
  }
}

impl Reducer {
  fn do_aggregate(&mut self, data: &Datum) -> Result<()> {
    let result =  match self.result {
      Some(v) => self.aggregator(v, data),
      None => data
    }?;

    self.result = Some(result);
    Ok(())
  }

  pub fn sum() -> Reducer {
    Reducer {
      result: None,
      aggregator: Datum::add_fuck
    }
  }
}

