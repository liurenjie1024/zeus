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
use util::errors::*;

struct AggExpr {
  agg_func_id: AggFuncId,
  args: Vec<Expr>
}

pub struct AggNode {
  group_bys: Vec<Expr>,
  aggs: Vec<AggExpr>,
  data: HashMap<Vec<u8>, Vec<Box<AggFunc>>>,
  input: Box<ExecNode>
}

impl ExecNode for AggNode {
  fn open(&mut self, context: &mut ExecContext) -> Result<()> {
    unimplemented!()
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

    let mut args: Vec<Expr> = Vec::new();
    agg_func.get_children().iter()
      .try_fold(&mut args, |res, expr| -> Result<&mut Vec<Expr>> {
        res.push(Expr::new(expr)?);
        Ok(res)
      });

    Ok(AggExpr {
      agg_func_id: agg_func.get_func_id(),
      args
    })
  }
}

impl AggNode {
  pub fn new(plan_node: &PlanNode, server_context: &ServerContext) -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::AGGREGATE_NODE,
      "Can create aggregate node from {:?}", plan_node.get_plan_node_type());
    ensure!(plan_node.get_children().len() == 1,
      "The children number of agg node should be 1 rather {}", plan_node.get_children().len());

    let input = plan_node.get_children().first().unwrap().to(server_context)?;

    let agg_node = plan_node.get_agg_node();

    let mut group_bys = Vec::new();
    agg_node.get_group_by().iter()
      .try_fold(&mut group_bys, |res, expr| -> Result<&mut Vec<Expr>> {
        group_bys.push(Expr::new(expr)?);
        Ok(res)
      });

    let mut aggs = Vec::new();
    agg_node.get_agg_func().iter()
      .try_fold(&mut aggs, |res, expr| -> Result<&mut Vec<AggExpr>> {
        res.push(AggExpr::new(expr)?);
        Ok(res)
      });

    Ok(box AggNode {
      group_bys,
      aggs,
      data: HashMap::new(),
      input
    })
  }
}

