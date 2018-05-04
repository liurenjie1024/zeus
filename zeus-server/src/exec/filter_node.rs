use std::vec::Vec;

use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use super::expression::Expr;
use super::ServerContext;
use super::ExecNode;
use super::ExecContext;
use super::Block;
use util::errors::*;

pub struct FilterNode {
  filters: Vec<Expr>,
  input: Box<ExecNode>
}

impl ExecNode for FilterNode {
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

impl FilterNode {
  pub fn new(plan_node: &PlanNode, server_context: &ServerContext) -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::FILTER_NODE,
      "Can't construct filter node from {:?}", plan_node.get_plan_node_type());
    ensure!(plan_node.get_children().len() == 1,
      "Filter node should only have 1 child rather {}", plan_node.get_children().len());

    let input = plan_node.get_children().first().unwrap()
      .to(server_context)?;

    let mut filters = Vec::new();

    plan_node.get_filter_node().get_conditions()
      .iter()
      .try_fold(&mut filters, |f, expr| Ok(f.push(Expr::new(expr)?)));

    Ok(box FilterNode {
      filters,
      input
    })
  }
}

