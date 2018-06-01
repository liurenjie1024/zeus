use std::vec::Vec;
use std::collections::HashMap;

use super::ServerContext;
use super::ExecNode;
use super::ExecContext;
use super::Block;
use super::expression::Expr;
use super::expression::agg_func::AggFunc;
use rpc::zeus_expr::AggFuncId;
use rpc::zeus_expr::Expression;
use rpc::zeus_expr::ExpressionType;
use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use storage::column::vec_column_data::Datum;
use util::errors::*;

struct AggExpr {
  _agg_func_id: AggFuncId,
  _args: Vec<Expr>
}

pub struct AggNode {
  _group_bys: Vec<Expr>,
  _aggs: Vec<AggExpr>,
  _data: HashMap<Vec<Datum>, Vec<Box<AggFunc>>>,
  input: Box<ExecNode>,
  executed: bool
}

impl ExecNode for AggNode {
  fn open(&mut self, context: &mut ExecContext) -> Result<()> {
    self.input.open(context)
  }

  fn next(&mut self) -> Result<Block> {
    unimplemented!()
  }

  fn close(&mut self) -> Result<()> {
    unimplemented!()
  }
}

impl AggExpr {
  fn new(expr: &Expression) -> Result<AggExpr> {
    ensure!(expr.get_expression_type() == ExpressionType::AGG_FUNCTION,
      "Expression type should be AGG_FUNCTION");

    let agg_func = expr.get_agg_func();

    let args = agg_func.get_children().iter()
      .try_fold(Vec::new(), |mut res, expr| -> Result<Vec<Expr>> {
        res.push(Expr::new(expr)?);
        Ok(res)
      })?;

    Ok(AggExpr {
      _agg_func_id: agg_func.get_func_id(),
      _args: args
    })
  }
}

impl AggNode {
  pub fn new(plan_node: &PlanNode, _server_context: &ServerContext, mut children: Vec<Box<ExecNode>>)
    -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::AGGREGATE_NODE,
      "Can create aggregate node from {:?}", plan_node.get_plan_node_type());
    ensure!(children.len() == 1,
      "The children number of agg node should be 1 rather {}", children.len());

    let input = children.pop().unwrap();

    let agg_node = plan_node.get_agg_node();

    let group_bys = agg_node.get_group_by().iter()
      .try_fold(Vec::new(), |mut res, expr| -> Result<Vec<Expr>> {
        res.push(Expr::new(expr)?);
        Ok(res)
      })?;

    let aggs = agg_node.get_agg_func().iter()
      .try_fold(Vec::new(), |mut res, expr| -> Result<Vec<AggExpr>> {
        res.push(AggExpr::new(expr)?);
        Ok(res)
      })?;

    Ok(box AggNode {
      _group_bys: group_bys,
      _aggs: aggs,
      _data: HashMap::new(),
      input,
      executed: false
    })
  }
}

