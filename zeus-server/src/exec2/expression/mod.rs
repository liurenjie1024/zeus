mod aggr;

use std::default::Default;

use arrow::datatypes::Field;
use arrow::datatypes::DataType;

use super::block::Block;
use rpc::zeus_meta::ColumnValue;
use rpc::zeus_expr::Expression;
use rpc::zeus_expr::ExpressionType;
use rpc::zeus_expr::ScalarFuncId;
use rpc::zeus_expr::AggFuncId;
use rpc::zeus_meta::ColumnType;
use util::errors::*;

pub use self::aggr::AggregationFunction;


pub enum Expr {
  Literal(LiteralExpr),
  ColumnRef(ColumnRefExpr),
  ScalarFunc(ScalarFuncExpr),
  Aggregation(AggregationExpr)
}

pub struct LiteralExpr {
  column_type: ColumnType,
  data: ColumnValue,
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

pub struct AggregationExpr {
  column_type: ColumnType,
  id: AggFuncId,
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
        data: rpc_expr.get_literal().get_value().clone(),
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
      ExpressionType::AGG_FUNCTION => {
        let args = rpc_expr.get_agg_func().get_children()
          .iter()
          .try_fold(Vec::new(), |mut res, expr| -> Result<Vec<Expr>> {
            res.push(Expr::new(expr)?);
            Ok(res)
          })?;

        Ok(Expr::Aggregation(AggregationExpr {
          id: rpc_expr.get_agg_func().get_func_id(),
          args,
          alias: rpc_expr.get_alias().to_string(),
          column_type: rpc_expr.get_field_type()
        }))

      }
    }
  }

  pub fn eval(&self, context: &EvalContext, input: &Block) -> Result<Block> {
    match self {
      Expr::Literal(ref literal) => {
        bail!("Literal can't be evaluated to block!")
      }
      Expr::ScalarFunc(ref scalar_func) => unimplemented!(),
      Expr::ColumnRef(ref column) => {
        input.get_column_by_name(column.column_name.as_str())
      },
      Expr::Aggregation(ref aggr) => unimplemented!()
    }
  }

  pub fn alias_ref(&self) -> &str {
    match self {
      Expr::Literal(literal) => literal.alias.as_str(),
      Expr::ColumnRef(column) => column.alias.as_str(),
      Expr::ScalarFunc(scalar_func) => scalar_func.alias.as_str(),
      Expr::Aggregation(aggr) => aggr.alias.as_str(),
    }
  }

  pub fn field_type(&self) -> ColumnType {
    match self {
      Expr::Literal(literal) => literal.column_type,
      Expr::ColumnRef(column) => column.column_type,
      Expr::ScalarFunc(scalar_func) => scalar_func.column_type,
      Expr::Aggregation(aggr) => aggr.column_type,
    }
  }
}

impl ScalarFuncExpr {
  pub fn new(column_type: ColumnType, func_id: ScalarFuncId, args: Vec<Expr>, alias: String) ->
    Result<ScalarFuncExpr> {
    Ok(Self {
      column_type,
      id: func_id,
      args,
      alias,
    })
  }

  pub fn get_id(&self) -> ScalarFuncId {
    self.id
  }

  pub fn get_args(&self) -> &[Expr] {
    self.args.as_slice()
  }
}

impl ColumnRefExpr {
  pub fn new(column_name: String, column_type: ColumnType, alias: String) -> Self {
    Self {
      column_type,
      column_name,
      alias
    }
  }

  pub fn get_column_name(&self) -> String {
    self.column_name.clone()
  }
}

impl LiteralExpr {
  pub fn new(column_type: ColumnType, data: ColumnValue, alias: String) -> Result<Self> {
    // TODO: Check column_type and data
    Ok(LiteralExpr {
      column_type,
      data,
      alias
    })
  }
}

impl AggregationExpr {
  pub fn args_slice(&self) -> &[Expr] {
    self.args.as_slice()
  }

  pub fn to_field(&self) -> Field {
    Field::new(
      self.alias.as_str(),
      self.column_type.to_arrow_data_type(),
      false)
  }
}

impl ColumnType {
  pub fn to_arrow_data_type(&self) -> DataType {
    match self {
      ColumnType::BOOL => DataType::Boolean,
      ColumnType::INT8 => DataType::Int8,
      ColumnType::INT16 => DataType::Int16,
      ColumnType::INT32 => DataType::Int32,
      ColumnType::INT64 => DataType::Int64,
      ColumnType::FLOAT4 => DataType::Float32,
      ColumnType::FLOAT8 => DataType::Float64,
      ColumnType::TIMESTAMP => DataType::Int64,
      ColumnType::STRING => DataType::Utf8
    }
  }
}


