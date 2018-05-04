use std::vec::Vec;

use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use super::ServerContext;
use super::ExecNode;
use super::expression::Expr;
use super::ExecContext;
use super::Block;
use util::errors::*;

pub struct ProjectNode {
  mappers: Vec<Expr>,
  input: Box<ExecNode>
}

impl ExecNode for ProjectNode {
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

impl ProjectNode {
  pub fn new(plan_node: &PlanNode, server_context: &ServerContext) -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::PROJECT_NODE,
      "Can't create project node from {:?}", plan_node.get_plan_node_type());
    ensure!(plan_node.get_children() == 1,
      "Project node's children size should be 1 rather {}", plan_node.get_children().len());

    let input = plan_node.get_children().first().unwrap().to(server_context)?;

    let mut mappers = Vec::new();

    plan_node.get_project_node().get_expressions()
      .iter()
      .try_fold(&mut mappers, |res, expr| Ok(res.push(Expr::new(expr)?)));

    Ok(box ProjectNode {
      mappers,
      input
    })
  }
}