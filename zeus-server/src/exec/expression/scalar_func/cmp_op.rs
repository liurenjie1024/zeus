use std::cmp::Ord;
use std::cmp::Ordering;

use storage::column::vec_column_data::{VecColumnData, Datum};
use storage::column::Column;
use exec::Block;
use rpc::zeus_meta::ColumnType;
use super::super::EvalContext;
use super::ScalarFunc;

use util::errors::*;

enum CmpOp {
  Less,
  Greater,
  LessEqual,
  GreaterEqual,
  Equal,
  NotEqual
}

impl CmpOp {
  fn is_match(&self, ordering: Ordering) -> bool {
    match self {
      &CmpOp::Less => ordering == Ordering::Less,
      &CmpOp::Greater => ordering == Ordering::Greater,
      &CmpOp::LessEqual => ordering != Ordering::Greater,
      &CmpOp::GreaterEqual => ordering != Ordering::Less,
      &CmpOp::Equal => ordering == Ordering::Equal,
      &CmpOp::NotEqual => ordering != Ordering::Equal
    }
  }
}


struct CmpOperator<T> {
  op: CmpOp,
  eval_fn: fn(&Datum) -> Result<T>
}

impl<T> ScalarFunc for CmpOperator<T>
  where T: Ord {
  fn eval(self, _ctx: &EvalContext, input: &Block) -> Result<Block> {
    ensure!(input.columns.len() >= 2, "Input block need at least 2 columns");
    let cmp_results = input.columns[0].iter()
      .zip(input.columns[1].iter())
      .try_fold(Vec::new(), |mut cmp_results, args| -> Result<Vec<Datum>> {
        let lhs = (self.eval_fn)(&args.0)?;
        let rhs = (self.eval_fn)(&args.1)?;

        cmp_results.push(Datum::Bool(self.op.is_match(lhs.cmp(&rhs))));
        Ok(cmp_results)
      })?;

    Ok(Block::from(vec![Column::new_vec(ColumnType::BOOL, VecColumnData::from(cmp_results))]))
  }
}

macro_rules! cmp_op_for {
  ($dt: ty, $eval_fn: path) => {
    impl CmpOperator<$dt> {
      fn lt() -> CmpOperator<$dt> {
        CmpOperator {
          op: CmpOp::Less,
          eval_fn: $eval_fn
        }
      }

      fn gt() -> CmpOperator<$dt> {
        CmpOperator {
          op: CmpOp::Greater,
          eval_fn: $eval_fn
        }
      }

      fn le() -> CmpOperator<$dt> {
        CmpOperator {
          op: CmpOp::LessEqual,
          eval_fn: $eval_fn
        }
      }

      fn ge() -> CmpOperator<$dt> {
        CmpOperator {
          op: CmpOp::GreaterEqual,
          eval_fn: $eval_fn
        }
      }

      fn eq() -> CmpOperator<$dt> {
        CmpOperator {
          op: CmpOp::Equal,
          eval_fn: $eval_fn
        }
      }

      fn ne() -> CmpOperator<$dt> {
        CmpOperator {
          op: CmpOp::NotEqual,
          eval_fn: $eval_fn
        }
      }
    }
  }
}

cmp_op_for!(bool, Datum::to_bool);
cmp_op_for!(i8, Datum::to_i8);
cmp_op_for!(i16, Datum::to_i16);
cmp_op_for!(i32, Datum::to_i32);
cmp_op_for!(i64, Datum::to_i64);
cmp_op_for!(f32, Datum::to_f32);
cmp_op_for!(f64, Datum::to_f64);
//cmp_op_for!(&str, Datum::to_str);

