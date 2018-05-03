use std::vec::Vec;

use super::ExecNode;
use super::expression::Expr;
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