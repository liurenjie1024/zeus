
use super::ScalarFunc;
use storage::column::Column;
use storage::column::vec_column_data::VecColumnData;
use storage::column::vec_column_data::Datum;
use exec::Block;
use exec::expression::EvalContext;
use rpc::zeus_meta::ColumnType;

use util::errors::*;

// AND, OR
pub struct ReducedLogicalOperator {
  reducer: fn(&Datum, &Datum) -> Result<Datum>
}

impl ScalarFunc for ReducedLogicalOperator {
  fn eval(self, _ctx: &EvalContext, input: &Block) -> Result<Column> {
    let mut ret = Vec::with_capacity(input.len());
    for i in 0..input.len() {
      let cur = input.columns.iter()
        .map(|c| c.get(i).unwrap())
        .try_fold(Datum::Bool(true), |left, right| -> Result<Datum> {
          (self.reducer)(&left, &right)
        })?;

      ret.push(cur);
    }

    let column = Column::new_vec(ColumnType::BOOL, VecColumnData::from(ret));
    Ok(column)
  }
}

impl ReducedLogicalOperator {
  pub fn and() -> ReducedLogicalOperator {
    ReducedLogicalOperator {
      reducer: Datum::and
    }
  }

  pub fn or() -> ReducedLogicalOperator {
    ReducedLogicalOperator {
      reducer: Datum::or
    }
  }
}

pub struct NotOperator {}

impl ScalarFunc for NotOperator {
  fn eval(self, ctx: &EvalContext, input: &Block) -> Result<Column> {
    let data_vec = input.columns_slice()[0].iter()
      .try_fold(Vec::new(), |mut data_vec, r| -> Result<Vec<Datum>> {
        data_vec.push(Datum::not(&r)?);
        Ok(data_vec)
      })?;

    let column = Column::new_vec(ColumnType::BOOL, VecColumnData::from(data_vec));
    Ok(column)
  }
}

pub struct LikeOperator {}

impl ScalarFunc for LikeOperator {
  fn eval(self, ctx: &EvalContext, input: &Block) -> Result<Column> {
    unimplemented!()
  }
}

