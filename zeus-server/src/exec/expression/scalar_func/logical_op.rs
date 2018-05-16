
use super::ScalarFunc;
use util::errors::*;
use storage::column::Column;
use storage::column::column_data::VecColumnData;
use storage::column::column_data::Datum;
use exec::Block;
use exec::ColumnWithInfo;
use exec::expression::EvalContext;
use rpc::zeus_meta::ColumnType;

// AND, OR
pub struct ReducedLogicalOperator {
  reducer: fn(&Datum, &Datum) -> Result<Datum>
}

impl ScalarFunc for ReducedLogicalOperator {
  fn eval(self, _ctx: &EvalContext, input: &Block) -> Result<Block> {
    let mut ret = Vec::with_capacity(input.len());
    for i in 0..input.len() {
      let cur = input.columns.iter()
        .map(|c| c.column.get(i).unwrap())
        .try_fold(Datum::Bool(true), |left, right| -> Result<Datum> {
          (self.reducer)(&left, &right)
        })?;

      ret.push(cur);
    }

    let column = Column::new(ColumnType::BOOL, VecColumnData::from(ret));
    Ok(Block::from(vec![ColumnWithInfo::from(column)]))
  }
}

impl ReducedLogicalOperator {
  pub fn and() -> ReducedLogicalOperator {
    ReducedLogicalOperator {
      reducer: Datum::and
    }
  }
}

