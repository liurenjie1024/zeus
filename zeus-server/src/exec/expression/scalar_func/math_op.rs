
use super::ScalarFunc;
use exec::expression::EvalContext;
use exec::Block;
use storage::column::ColumnBuilder;
use storage::column::Column;
use rpc::zeus_meta::ColumnType;
use storage::column::vec_column_data::Datum;
use util::errors::*;

pub struct MathOperator {
  result_type: ColumnType,
  math_op: fn(&Datum, &Datum) -> Result<Datum>
}

impl ScalarFunc for MathOperator {
  fn eval(self, ctx: &EvalContext, input: &Block) -> Result<Column> {
    let vec_data = input.columns_slice()[0].iter()
      .zip(input.columns_slice()[1].iter())
      .try_fold(Vec::new(), |mut ret, pair| -> Result<Vec<Datum>> {
        ret.push((self.math_op)(&pair.0, &pair.1)?);
        Ok(ret)
      })?;

    let column = ColumnBuilder::new_vec(self.result_type, vec_data)
      .build();

    Ok(column)
  }
}

impl MathOperator {
  pub fn add(result_type: ColumnType) -> MathOperator {
    MathOperator {
      result_type,
      math_op: Datum::add
    }
  }

  pub fn minus(result_type: ColumnType) -> MathOperator {
    MathOperator {
      result_type,
      math_op: Datum::minus
    }
  }

  pub fn multiply(result_type: ColumnType) -> MathOperator {
    MathOperator {
      result_type,
      math_op: Datum::multiply
    }
  }

  pub fn divide(result_type: ColumnType) -> MathOperator {
    MathOperator {
      result_type,
      math_op: Datum::divide
    }
  }
}
