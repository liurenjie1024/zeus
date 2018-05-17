pub mod logical_op;
pub mod cmp_op;

use super::EvalContext;
use super::ScalarFuncExpr;
use storage::column::Column;
use exec::Block;
use rpc::zeus_expr::ScalarFuncId;
use self::logical_op::ReducedLogicalOperator;
use util::errors::*;

trait ScalarFunc {
  fn eval(self, ctx: &EvalContext, input: &Block) -> Result<Block>;
}

impl ScalarFuncExpr {
  pub fn eval(&mut self, ctx: &EvalContext, input: &Block) -> Result<Block> {
    let columns = self.args.iter_mut()
      .try_fold(Vec::new(), |mut columns, arg| -> Result<Vec<Column>> {
        let mut block = arg.eval(ctx, input)?;
        columns.append(&mut block.columns);
        Ok(columns)
      })?;

    let args = Block::from(columns);
    match self.id {
      ScalarFuncId::AND => ScalarFuncExpr::eval_operator(ReducedLogicalOperator::and(), ctx, &args),
      ScalarFuncId::ADD_INT4_INT4 => unreachable!()
    }
  }

  fn eval_operator<F: ScalarFunc>(op: F, ctx: &EvalContext, args: &Block) -> Result<Block> {
    op.eval(ctx, args)
  }
}



