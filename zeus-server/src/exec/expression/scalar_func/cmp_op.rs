use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::fmt::Display;

use storage::column::vec_column_data::{VecColumnData, Datum};
use storage::column::Column;
use exec::Block;
use rpc::zeus_meta::ColumnType;
use super::super::EvalContext;
use super::ScalarFunc;

use util::errors::*;

pub enum CmpOp {
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

pub struct CmpOperator {
  op: CmpOp,
  order_fn: fn(&Datum, &Datum) -> Result<Ordering>
}

impl ScalarFunc for CmpOperator {
  fn eval(self, _ctx: &EvalContext, input: &Block) -> Result<Column>
  {
    ensure!(input.columns.len() >= 2, "Input block need at least 2 columns");
    let cmp_results = input.columns[0].iter()
      .zip(input.columns[1].iter())
      .try_fold(Vec::new(), move |mut cmp_results, args| -> Result<Vec<Datum>> {
        let order = (self.order_fn)(&args.0, &args.1)?;
        cmp_results.push(Datum::Bool(self.op.is_match(order)));
        Ok(cmp_results)
      })?;

    Ok(Column::new_vec(ColumnType::BOOL, VecColumnData::from(cmp_results)))
  }
}

impl CmpOperator {
  fn order_of_copy<F, T>(left: &Datum, right: &Datum, eval_fn: F) -> Result<Ordering>
    where T: Copy + PartialOrd + Display,
          F: Fn(&Datum) -> Result<T>
  {
    let lhs = eval_fn(left)?;
    let rhs = eval_fn(right)?;

    lhs.partial_cmp(&rhs).ok_or_else(
      move || ErrorKind::UnableToCompare(format!("Unable to compare:{}, {}", lhs, rhs)).into())
  }

  fn order_of_ref<F, T>(left: &Datum, right: &Datum, eval_fn: F) -> Result<Ordering>
    where T: ?Sized + PartialOrd + Display,
          F: for<'a> Fn(&'a Datum) -> Result<&'a T>
  {
    let lhs = eval_fn(left)?;
    let rhs = eval_fn(right)?;

    lhs.partial_cmp(&rhs).ok_or_else(
      || ErrorKind::UnableToCompare("Unable to compare".to_string()).into())
  }

  fn new(op: CmpOp,
    order_fn: fn(&Datum, &Datum) -> Result<Ordering>) -> impl ScalarFunc {
    CmpOperator {
      op,
      order_fn
    }
  }

//  pub fn bool_cmp_operator(op: CmpOp) -> impl ScalarFunc {
//    CmpOperator::new(op,
//      |left, right| CmpOperator::order_of_copy(left, right, Datum::to_bool))
//  }
//
//  pub fn i8_cmp_operator(op: CmpOp) -> impl ScalarFunc {
//    CmpOperator::new(op,
//      |left, right| CmpOperator::order_of_copy(left, right, Datum::to_i8))
//  }
//
//  pub fn i16_cmp_operator(op: CmpOp) -> impl ScalarFunc {
//    CmpOperator::new(op,
//     |left, right| CmpOperator::order_of_copy(left, right, Datum::to_i16))
//  }

  pub fn i32_cmp_operator(op: CmpOp) -> impl ScalarFunc {
    CmpOperator::new(op,
      |left, right| CmpOperator::order_of_copy(left, right, Datum::to_i32))
  }

  pub fn i64_cmp_operator(op: CmpOp) -> impl ScalarFunc {
    CmpOperator::new(op,
      |left, right| CmpOperator::order_of_copy(left, right, Datum::to_i64))
  }

  pub fn f4_cmp_operator(op: CmpOp) -> impl ScalarFunc {
    CmpOperator::new(op,
      |left, right| CmpOperator::order_of_copy(left, right, Datum::to_f32))
  }

  pub fn f8_cmp_operator(op: CmpOp) -> impl ScalarFunc {
    CmpOperator::new(op,
      |left, right| CmpOperator::order_of_copy(left, right, Datum::to_f64))
  }

  pub fn str_cmp_operator(op: CmpOp) -> impl ScalarFunc {
    CmpOperator::new(op,
      |left, right| CmpOperator::order_of_ref(left, right, Datum::to_str))
  }
}




