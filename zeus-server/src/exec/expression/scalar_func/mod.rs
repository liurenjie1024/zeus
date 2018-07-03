pub mod logical_op;
pub mod cmp_op;
pub mod math_op;

use super::EvalContext;
use super::ScalarFuncExpr;
use storage::column::Column;
use exec::Block;
use rpc::zeus_expr::ScalarFuncId;
use rpc::zeus_meta::ColumnType;
use self::logical_op::ReducedLogicalOperator;
use self::logical_op::NotOperator;
use self::logical_op::LikeOperator;
use self::cmp_op::{CmpOperator, CmpOp};
use self::math_op::MathOperator;
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
  AND => ReducedLogicalOperator::and(),
  OR => ReducedLogicalOperator::or(),
  NOT => NotOperator {},
  LIKE => LikeOperator {},

  GT_INT32 => CmpOperator::i32_cmp_operator(CmpOp::Greater),
  GT_INT64 => CmpOperator::i64_cmp_operator(CmpOp::Greater),
  GT_FLOAT4 => CmpOperator::f4_cmp_operator(CmpOp::Greater),
  GT_FLOAT8 => CmpOperator::f8_cmp_operator(CmpOp::Greater),
  GT_STR => CmpOperator::str_cmp_operator(CmpOp::Greater),

  GE_INT32 => CmpOperator::i32_cmp_operator(CmpOp::GreaterEqual),
  GE_INT64 => CmpOperator::i64_cmp_operator(CmpOp::GreaterEqual),
  GE_FLOAT4 => CmpOperator::f4_cmp_operator(CmpOp::GreaterEqual),
  GE_FLOAT8 => CmpOperator::f8_cmp_operator(CmpOp::GreaterEqual),
  GE_STR => CmpOperator::str_cmp_operator(CmpOp::GreaterEqual),

  LT_INT32 => CmpOperator::i32_cmp_operator(CmpOp::Less),
  LT_INT64 => CmpOperator::i64_cmp_operator(CmpOp::Less),
  LT_FLOAT4 => CmpOperator::f4_cmp_operator(CmpOp::Less),
  LT_FLOAT8 => CmpOperator::f8_cmp_operator(CmpOp::Less),
  LT_STR => CmpOperator::str_cmp_operator(CmpOp::Less),

  LE_INT32 => CmpOperator::i32_cmp_operator(CmpOp::LessEqual),
  LE_INT64 => CmpOperator::i64_cmp_operator(CmpOp::LessEqual),
  LE_FLOAT4 => CmpOperator::f4_cmp_operator(CmpOp::LessEqual),
  LE_FLOAT8 => CmpOperator::f8_cmp_operator(CmpOp::LessEqual),
  LE_STR => CmpOperator::str_cmp_operator(CmpOp::LessEqual),

  EQ_INT32 => CmpOperator::i32_cmp_operator(CmpOp::Equal),
  EQ_INT64 => CmpOperator::i64_cmp_operator(CmpOp::Equal),
  EQ_FLOAT4 => CmpOperator::f4_cmp_operator(CmpOp::Equal),
  EQ_FLOAT8 => CmpOperator::f8_cmp_operator(CmpOp::Equal),
  EQ_STR => CmpOperator::str_cmp_operator(CmpOp::Equal),

  NE_INT32 => CmpOperator::i32_cmp_operator(CmpOp::NotEqual),
  NE_INT64 => CmpOperator::i64_cmp_operator(CmpOp::NotEqual),
  NE_FLOAT4 => CmpOperator::f4_cmp_operator(CmpOp::NotEqual),
  NE_FLOAT8 => CmpOperator::f8_cmp_operator(CmpOp::NotEqual),
  NE_STR => CmpOperator::str_cmp_operator(CmpOp::NotEqual),

  ADD_INT32 => MathOperator::add(ColumnType::INT32),
  ADD_INT64 => MathOperator::add(ColumnType::INT64),
  ADD_FLOAT4 => MathOperator::add(ColumnType::FLOAT4),
  ADD_FLOAT8 => MathOperator::add(ColumnType::FLOAT8),

  MINUS_INT32 => MathOperator::minus(ColumnType::INT32),
  MINUS_INT64 => MathOperator::minus(ColumnType::INT64),
  MINUS_FLOAT4 => MathOperator::minus(ColumnType::FLOAT4),
  MINUS_FLOAT8 => MathOperator::minus(ColumnType::FLOAT8),

  MUL_INT32 => MathOperator::multiply(ColumnType::INT32),
  MUL_INT64 => MathOperator::multiply(ColumnType::INT64),
  MUL_FLOAT4 => MathOperator::multiply(ColumnType::FLOAT4),
  MUL_FLOAT8 => MathOperator::multiply(ColumnType::FLOAT8),

  DIV_INT32 => MathOperator::divide(ColumnType::INT32),
  DIV_INT64 => MathOperator::divide(ColumnType::INT64),
  DIV_FLOAT4 => MathOperator::divide(ColumnType::FLOAT4),
  DIV_FLOAT8 => MathOperator::divide(ColumnType::FLOAT8),
}

