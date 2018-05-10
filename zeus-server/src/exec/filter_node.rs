use std::vec::Vec;

use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use super::expression::Expr;
use super::ServerContext;
use super::ExecNode;
use super::ExecContext;
use super::Block;
use util::errors::*;

pub struct FilterExecNode {
  _filter: Option<Expr> ,
  input: Box<ExecNode>
}

impl ExecNode for FilterExecNode {
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

impl FilterExecNode {
  pub fn new(plan_node: &PlanNode, _server_context: &ServerContext, mut children: Vec<Box<ExecNode>>)
    -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::FILTER_NODE,
      "Can't construct filter node from {:?}", plan_node.get_plan_node_type());
    ensure!(children.len() == 1,
      "Filter node should only have 1 child rather {}", children.len());
    ensure!(plan_node.get_filter_node().get_conditions().len() == 1,
      "Filter node condition can only support one condition");


    let input = children.pop().unwrap();

    let filters = plan_node.get_filter_node().get_conditions()
      .first()
      .map(Expr::new)
      .map_or(Ok(None), |t| t.map(Some))?;

    Ok(box FilterExecNode {
      _filter: filters,
      input
    })
  }
}

