
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
  math_op: fn(&Datum, &Datum) -> Result<Datum>,
  eval_fn: fn(&Datum) -> Result<Datum>
}

impl ScalarFunc for MathOperator {
  fn eval(self, _ctx: &EvalContext, input: &Block) -> Result<Column> {
    let vec_data = input.columns_slice()[0].iter()
      .zip(input.columns_slice()[1].iter())
      .try_fold(Vec::new(), |mut ret, pair| -> Result<Vec<Datum>> {
        let left = eval_fn(pair.0)?;
        let right = eval_fn(pair.1)?;
        ret.push((self.math_op)(&left, &right)?);
        Ok(ret)
      })?;

    let column = ColumnBuilder::new_vec(self.result_type, vec_data)
      .build();

    Ok(column)
  }
}

impl MathOperator {
  pub fn add_i32() -> MathOperator {
    MathOperator {
      result_type: ColumnType::INT32,
      math_op: Datum::add,
      eval_fn: |r| r.to_i32().map(|x| x.into())
    }
  }

  pub fn add_i64() -> MathOperator {
    MathOperator {
      result_type: ColumnType::INT64,
      math_op: Datum::add,
      eval_fn: |r| r.to_i64().map(|x| x.into())
    }
  }

  pub fn add_f4() -> MathOperator {
    MathOperator {
      result_type: ColumnType::FLOAT4,
      math_op: Datum::add,
      eval_fn: |r| r.to_f32().map(|x| x.into())
    }
  }

  pub fn add_f8() -> MathOperator {
    MathOperator {
      result_type: ColumnType::FLOAT8,
      math_op: Datum::add,
      eval_fn: |r| r.to_f64().map(|x| x.into())
    }
  }

  pub fn minus_i32() -> MathOperator {
    MathOperator {
      result_type: ColumnType::INT32,
      math_op: Datum::minus,
      eval_fn: |r| r.to_i32().map(|x| x.into())
    }
  }

  pub fn minus_i64() -> MathOperator {
    MathOperator {
      result_type: ColumnType::INT64,
      math_op: Datum::minus,
      eval_fn: |r| r.to_i64().map(|x| x.into())
    }
  }

  pub fn minus_f4() -> MathOperator {
    MathOperator {
      result_type: ColumnType::FLOAT4,
      math_op: Datum::minus,
      eval_fn: |r| r.to_f32().map(|x| x.into())
    }
  }

  pub fn minus_f8() -> MathOperator {
    MathOperator {
      result_type: ColumnType::FLOAT8,
      math_op: Datum::minus,
      eval_fn: |r| r.to_f64().map(|x| x.into())
    }
  }

  pub fn multiply_i32() -> MathOperator {
    MathOperator {
      result_type: ColumnType::INT32,
      math_op: Datum::multiply,
      eval_fn: |r| r.to_i32().map(|x| x.into())
    }
  }

  pub fn multiply_i64() -> MathOperator {
    MathOperator {
      result_type: ColumnType::INT64,
      math_op: Datum::multiply,
      eval_fn: |r| r.to_i64().map(|x| x.into())
    }
  }

  pub fn multiply_f4() -> MathOperator {
    MathOperator {
      result_type: ColumnType::FLOAT4,
      math_op: Datum::multiply,
      eval_fn: |r| r.to_f32().map(|x| x.into())
    }
  }

  pub fn multiply_f8() -> MathOperator {
    MathOperator {
      result_type: ColumnType::FLOAT8,
      math_op: Datum::multiply,
      eval_fn: |r| r.to_f64().map(|x| x.into())
    }
  }

  pub fn divide_i32() -> MathOperator {
    MathOperator {
      result_type: ColumnType::INT32,
      math_op: Datum::divide,
      eval_fn: |r| r.to_i32().map(|x| x.into())
    }
  }

  pub fn divide_i64() -> MathOperator {
    MathOperator {
      result_type: ColumnType::INT64,
      math_op: Datum::divide,
      eval_fn: |r| r.to_i64().map(|x| x.into())
    }
  }

  pub fn divide_f4() -> MathOperator {
    MathOperator {
      result_type: ColumnType::FLOAT4,
      math_op: Datum::divide,
      eval_fn: |r| r.to_f32().map(|x| x.into())
    }
  }

  pub fn divide_f8() -> MathOperator {
    MathOperator {
      result_type: ColumnType::FLOAT8,
      math_op: Datum::divide,
      eval_fn: |r| r.to_f64().map(|x| x.into())
    }
  }
}
