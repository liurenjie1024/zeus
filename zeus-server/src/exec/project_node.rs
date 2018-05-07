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
  _mappers: Vec<Expr>,
  input: Box<ExecNode>
}

impl ExecNode for ProjectNode {
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

impl ProjectNode {
  pub fn new(plan_node: &PlanNode, _server_context: &ServerContext, mut children: Vec<Box<ExecNode>>)
    -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::PROJECT_NODE,
      "Can't create project node from {:?}", plan_node.get_plan_node_type());
    ensure!(children.len() == 1,
      "Project node's children size should be 1 rather {}", children.len());

    let input = children.pop().unwrap();

    let mappers = plan_node.get_project_node().get_expressions()
      .iter()
      .try_fold(Vec::new(), |mut res, expr| -> Result<Vec<Expr>> {
        res.push(Expr::new(expr)?);
        Ok(res)
      })?;

    Ok(box ProjectNode {
      _mappers: mappers,
      input
    })
  }
}