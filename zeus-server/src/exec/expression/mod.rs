pub mod scalar_func;
pub mod agg_func;

use std::default::Default;

use storage::column::vec_column_data::Datum;
use storage::column::Column;
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
  _column_name: String
}

pub struct ScalarFuncExpr {
  id: ScalarFuncId,
  _args: Vec<Expr>
}

pub struct EvalContext {
}

impl Default for EvalContext {
  fn default() -> Self {
    EvalContext {}
  }
}

impl Expr {
  pub fn new(rpc_expr: &Expression) -> Result<Expr> {
    match rpc_expr.expression_type {
      ExpressionType::LITERAL => Ok(Expr::Literal(LiteralExpr {
        column_type: rpc_expr.get_literal().get_field_type(),
        data: Datum::from(rpc_expr.get_literal())
      })),
      ExpressionType::COLUMN_REF => Ok(Expr::ColumnRef(ColumnRefExpr {
        _column_name: rpc_expr.get_column().get_name().to_string()
      })),
      ExpressionType::SCALAR_FUNCTION => {
        let args = rpc_expr.get_scalar_func().get_children()
          .iter()
          .try_fold(Vec::new(), |mut res, expr| -> Result<Vec<Expr>> {
            res.push(Expr::new(expr)?);
            Ok(res)
          })?;

        Ok(Expr::ScalarFunc(ScalarFuncExpr {
          id: rpc_expr.get_scalar_func().get_func_id(),
          _args: args
        }))
      },
      ExpressionType::AGG_FUNCTION => bail!("Aggregation Function can't be constructed from expr")
    }
  }

  pub fn eval(&mut self, _context: &EvalContext, input: &Block) -> Result<Block> {
    match self {
      Expr::Literal(ref literal) => {
        let column = Column::new_const(literal.column_type, literal.data.clone(), input.len());
        Ok(Block::from(vec![column]))
      }
      Expr::ScalarFunc(ref mut scalar_func)  => scalar_func.eval(_context, input),
      _ => unimplemented!()
    }
  }
}

