pub mod logical_op;

use super::EvalContext;
use super::ScalarFuncExpr;
use exec::Block;
use rpc::zeus_expr::ScalarFuncId;
use self::logical_op::ReducedLogicalOperator;
use util::errors::*;

trait ScalarFunc {
  fn eval(self, ctx: &EvalContext, input: &Block) -> Result<Block>;
}

impl ScalarFuncExpr {
  pub fn eval(&self, ctx: &EvalContext, input: &Block) -> Result<Block> {
    match self.id {
      ScalarFuncId::AND => ScalarFuncExpr::eval_operator(ReducedLogicalOperator::and(), ctx, input),
      ScalarFuncId::ADD_INT4_INT4 => unreachable!()
    }
  }

  fn eval_operator<F: ScalarFunc>(op: F, ctx: &EvalContext, args: &Block) -> Result<Block> {
    op.eval(ctx, args)
  }
}



