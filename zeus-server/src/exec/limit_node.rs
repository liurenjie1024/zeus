
use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use super::ServerContext;
use super::ExecNode;
use super::ExecContext;
use super::Block;
use util::errors::*;

pub struct LimitNode {
  limit: i32,
  input: Box<ExecNode>,
  cur: i32
}

impl ExecNode for LimitNode  {
  fn open(&mut self, context: &mut ExecContext) -> Result<()> {
    self.input.open(context)?;
    self.cur = 0;
    Ok(())
  }

  fn next(&mut self) -> Result<Block> {
    if self.cur >= self.limit {
      Ok(Block::empty_block())
    } else {
      let ret = self.input.next()?;
      let ret_len = ret.len();
      let left = (self.limit - self.cur) as usize;
      if ret_len >= left  {
        self.cur = self.limit;
        Ok(ret.take(left))
      } else {
        self.cur += ret_len as i32;
        Ok(ret)
      }
    }
  }

  fn close(&mut self) -> Result<()> {
    self.input.close()?;
    self.cur = 0;
    Ok(())
  }
}

impl LimitNode {
  pub fn new(plan_node: &PlanNode, server_context: &ServerContext) -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() != PlanNodeType::LIMIT_NODE,
      "Can't build limit node from plan node type: {:?}", plan_node.get_plan_node_type());
    ensure!(plan_node.get_children().len() == 1,
      "Invalid number of inputs of limit node: {}", plan_node.get_children().len());

    let input = plan_node.get_children().first().unwrap().to(server_context)?;

    Ok(box LimitNode {
      limit: plan_node.get_limit_node().get_limit(),
      input,
      cur: 0
    })
  }
}