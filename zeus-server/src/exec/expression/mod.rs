pub mod scalar_func;
pub mod agg_func;

use storage::column::column_data::Datum;
use exec::Block;
use rpc::zeus_expr::Expression;
use rpc::zeus_expr::ExpressionType;
use rpc::zeus_expr::ScalarFuncId;
use rpc::zeus_meta::ColumnType;
use util::errors::*;

pub enum Expr {
  Literal(LiteralExpr),
  ColumnRef(ColumnRefExpr),
  ScalarFunc(ScalarFuncExpr)
}

pub struct LiteralExpr {
  column_type: ColumnType,
  data: Datum
}

pub struct ColumnRefExpr {
  column_name: String
}

pub struct ScalarFuncExpr {
  id: ScalarFuncId,
  args: Vec<Expr>
}

pub struct EvalContext {
}

impl Expr {
  pub fn new(rpc_expr: &Expression) -> Result<Expr> {
    match rpc_expr.expression_type {
      ExpressionType::LITERAL => Ok(Expr::Literal(LiteralExpr {
        column_type: rpc_expr.get_literal().get_field_type(),
        data: Datum::from(rpc_expr.get_literal())
      })),
      ExpressionType::COLUMN_REF => Ok(Expr::ColumnRef(ColumnRefExpr {
        column_name: rpc_expr.get_column().get_name().to_string()
      })),
      ExpressionType::SCALA_FUNCTION => {
        let mut args = Vec::new();
        rpc_expr.get_scalar_func().get_children()
          .iter()
          .try_fold(&mut args, |res, expr| Ok(res.push(Expr::new(expr)?)));

        Ok(Expr::ScalarFunc(ScalarFuncExpr {
          id: rpc_expr.get_scalar_func().get_func_id(),
          args
        }))
      },
      ExpressionType::AGG_FUNCTION => bail!("Aggregation Function can't be constructed from expr")
    }
  }

  pub fn eval(&mut self, context: &EvalContext, input: &Block) -> Block {
    unimplemented!()
  }
}

