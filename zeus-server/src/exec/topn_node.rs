use std::vec::Vec;

use super::ExecNode;
use super::ExecContext;
use super::Block;
use super::expression::Expr;
use util::errors::*;

struct SortItem {
  items: Vec<Expr>,
  desc: bool
}

pub struct TopNNode {
  sort_items: Vec<SortItem>,
  limit: i32,
  input: Box<ExecNode>
}

impl ExecNode for TopNNode {
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

