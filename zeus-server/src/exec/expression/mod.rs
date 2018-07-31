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
  data: Datum,
  alias: String
}

pub struct ColumnRefExpr {
  column_type: ColumnType,
  column_name: String,
  alias: String
}

pub struct ScalarFuncExpr {
  column_type: ColumnType,
  id: ScalarFuncId,
  args: Vec<Expr>,
  alias: String
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
        column_type: rpc_expr.get_field_type(),
        data: Datum::new_literal_expr(rpc_expr.get_literal(), rpc_expr.get_field_type()),
        alias: rpc_expr.get_alias().to_string(),
      })),
      ExpressionType::COLUMN_REF => Ok(Expr::ColumnRef(ColumnRefExpr {
        column_name: rpc_expr.get_column().get_name().to_string(),
        alias: rpc_expr.get_alias().to_string(),
        column_type: rpc_expr.get_field_type()
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
          args,
          alias: rpc_expr.get_alias().to_string(),
          column_type: rpc_expr.get_field_type()
        }))
      },
      ExpressionType::AGG_FUNCTION => bail!("Aggregation Function can't be constructed from expr")
    }
  }

  pub fn eval(&self, context: &EvalContext, input: &Block) -> Result<Column> {
    match self {
      Expr::Literal(ref literal) => {
        let column = Column::new_const(literal.column_type, literal.data.clone(), input.len());
        Ok(column)
      }
      Expr::ScalarFunc(ref scalar_func)  => scalar_func.eval(context, input),
      Expr::ColumnRef(ref column) => {
        let column = input.column_by_name(column.column_name.as_str())
          .ok_or(ErrorKind::ColumnNameNotFound(column.column_name.clone()))?;

        Ok(column)
      }
    }
  }

  pub fn alias_ref(&self) -> &str {
    match self {
      Expr::Literal(literal) => literal.alias.as_str(),
      Expr::ColumnRef(column) => column.alias.as_str(),
      Expr::ScalarFunc(scalar_func) => scalar_func.alias.as_str()
    }
  }

  pub fn field_type(&self) -> ColumnType {
    match self {
      Expr::Literal(literal) => literal.column_type,
      Expr::ColumnRef(column) => column.column_type,
      Expr::ScalarFunc(scalar_func) => scalar_func.column_type
    }
  }
}

impl ScalarFuncExpr {
  pub fn get_id(&self) -> ScalarFuncId {
    self.id
  }

  pub fn get_args(&self) -> &[Expr] {
    self.args.as_slice()
  }
}

impl ColumnRefExpr {
  pub fn get_column_name(&self) -> String {
    self.column_name.clone()
  }
}

impl LiteralExpr {
  pub fn get_data(&self) -> Datum {
    self.data.clone()
  }
}
