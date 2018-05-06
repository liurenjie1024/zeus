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
  _filters: Vec<Expr>,
  input: Box<ExecNode>
}

impl ExecNode for FilterNode {
  fn open(&mut self, _context: &mut ExecContext) -> Result<()> {
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

    let filters = plan_node.get_filter_node().get_conditions()
      .iter()
      .try_fold(Vec::new(), |mut f, expr| -> Result<Vec<Expr>> {
        f.push(Expr::new(expr)?);
        Ok(f)
      })?;

    Ok(box FilterNode {
      _filters: filters,
      input
    })
  }
}

