use std::vec::Vec;

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