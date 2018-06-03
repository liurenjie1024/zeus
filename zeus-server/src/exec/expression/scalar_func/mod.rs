pub mod logical_op;
pub mod cmp_op;

use super::EvalContext;
use super::ScalarFuncExpr;
use storage::column::Column;
use exec::Block;
use rpc::zeus_expr::ScalarFuncId;
use self::logical_op::ReducedLogicalOperator;
use self::cmp_op::{CmpOperator, CmpOp};
use util::errors::*;

pub trait ScalarFunc {
  fn eval(self, ctx: &EvalContext, input: &Block) -> Result<Column>;
}

impl ScalarFuncExpr {
  pub fn eval(&self, ctx: &EvalContext, input: &Block) -> Result<Column> {
    let columns = self.args.iter()
      .try_fold(Vec::new(), |mut columns, arg| -> Result<Vec<Column>> {
        let column = arg.eval(ctx, input)?;
        columns.push(column);
        Ok(columns)
      })?;

    let args = Block::from(columns);

    dispatch_scalar_func_call(self.id, &ctx, &args)
  }

  fn eval_operator<F: ScalarFunc>(op: F, ctx: &EvalContext, args: &Block) -> Result<Column> {
    op.eval(ctx, args)
  }
}

macro_rules! dispatch {
  ($($scalar_func_id: ident => $scalar_func: expr,)*) => {
    fn dispatch_scalar_func_call(id: ScalarFuncId, ctx: &EvalContext, args: &Block)
      -> Result<Column> {
      match id {
        $(
          ScalarFuncId::$scalar_func_id => ScalarFuncExpr::eval_operator($scalar_func, ctx, args),
        )*
      }
    }
  }
}

dispatch! {
  ADD_INT4_INT4 => ReducedLogicalOperator::and(),
  AND => ReducedLogicalOperator::and(),

  GT_BOOL => CmpOperator::bool_cmp_operator(CmpOp::Greater),
  GT_I8 => CmpOperator::i8_cmp_operator(CmpOp::Greater),
  GT_I16 => CmpOperator::i16_cmp_operator(CmpOp::Greater),
  GT_I32 => CmpOperator::i32_cmp_operator(CmpOp::Greater),
  GT_I64 => CmpOperator::i64_cmp_operator(CmpOp::Greater),
  GT_F4 => CmpOperator::f4_cmp_operator(CmpOp::Greater),
  GT_F8 => CmpOperator::f8_cmp_operator(CmpOp::Greater),
  GT_STR => CmpOperator::str_cmp_operator(CmpOp::Greater),

  GE_BOOL => CmpOperator::bool_cmp_operator(CmpOp::GreaterEqual),
  GE_I8 => CmpOperator::i8_cmp_operator(CmpOp::GreaterEqual),
  GE_I16 => CmpOperator::i16_cmp_operator(CmpOp::GreaterEqual),
  GE_I32 => CmpOperator::i32_cmp_operator(CmpOp::GreaterEqual),
  GE_I64 => CmpOperator::i64_cmp_operator(CmpOp::GreaterEqual),
  GE_F4 => CmpOperator::f4_cmp_operator(CmpOp::GreaterEqual),
  GE_F8 => CmpOperator::f8_cmp_operator(CmpOp::GreaterEqual),
  GE_STR => CmpOperator::str_cmp_operator(CmpOp::GreaterEqual),

  LT_BOOL => CmpOperator::bool_cmp_operator(CmpOp::Less),
  LT_I8 => CmpOperator::i8_cmp_operator(CmpOp::Less),
  LT_I16 => CmpOperator::i16_cmp_operator(CmpOp::Less),
  LT_I32 => CmpOperator::i32_cmp_operator(CmpOp::Less),
  LT_I64 => CmpOperator::i64_cmp_operator(CmpOp::Less),
  LT_F4 => CmpOperator::f4_cmp_operator(CmpOp::Less),
  LT_F8 => CmpOperator::f8_cmp_operator(CmpOp::Less),
  LT_STR => CmpOperator::str_cmp_operator(CmpOp::Less),

  LE_BOOL => CmpOperator::bool_cmp_operator(CmpOp::LessEqual),
  LE_I8 => CmpOperator::i8_cmp_operator(CmpOp::LessEqual),
  LE_I16 => CmpOperator::i16_cmp_operator(CmpOp::LessEqual),
  LE_I32 => CmpOperator::i32_cmp_operator(CmpOp::LessEqual),
  LE_I64 => CmpOperator::i64_cmp_operator(CmpOp::LessEqual),
  LE_F4 => CmpOperator::f4_cmp_operator(CmpOp::LessEqual),
  LE_F8 => CmpOperator::f8_cmp_operator(CmpOp::LessEqual),
  LE_STR => CmpOperator::str_cmp_operator(CmpOp::LessEqual),

  EQ_BOOL => CmpOperator::bool_cmp_operator(CmpOp::Equal),
  EQ_I8 => CmpOperator::i8_cmp_operator(CmpOp::Equal),
  EQ_I16 => CmpOperator::i16_cmp_operator(CmpOp::Equal),
  EQ_I32 => CmpOperator::i32_cmp_operator(CmpOp::Equal),
  EQ_I64 => CmpOperator::i64_cmp_operator(CmpOp::Equal),
  EQ_F4 => CmpOperator::f4_cmp_operator(CmpOp::Equal),
  EQ_F8 => CmpOperator::f8_cmp_operator(CmpOp::Equal),
  EQ_STR => CmpOperator::str_cmp_operator(CmpOp::Equal),

  NE_BOOL => CmpOperator::bool_cmp_operator(CmpOp::NotEqual),
  NE_I8 => CmpOperator::i8_cmp_operator(CmpOp::NotEqual),
  NE_I16 => CmpOperator::i16_cmp_operator(CmpOp::NotEqual),
  NE_I32 => CmpOperator::i32_cmp_operator(CmpOp::NotEqual),
  NE_I64 => CmpOperator::i64_cmp_operator(CmpOp::NotEqual),
  NE_F4 => CmpOperator::f4_cmp_operator(CmpOp::NotEqual),
  NE_F8 => CmpOperator::f8_cmp_operator(CmpOp::NotEqual),
  NE_STR => CmpOperator::str_cmp_operator(CmpOp::NotEqual),
}

