
use super::ExecNode;
use super::ExecContext;
use super::Block;
use util::errors::*;

pub struct LimitNode {
  limit: i32,
  input: Box<ExecNode>
}

impl ExecNode for LimitNode  {
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