use std::vec::Vec;
use std::collections::HashMap;

use super::ExecNode;
use super::ExecContext;
use super::Block;
use super::expression::Expr;
use super::expression::agg_func::AggFunc;
use util::errors::*;

struct AggExpr {
  args: Vec<Expr>
}

pub struct AggNode {
  group_bys: Vec<Expr>,
  aggs: Vec<AggExpr>,
  data: HashMap<Vec<u8>, Vec<Box<AggFunc>>>,
  input: Box<ExecNode>
}

impl ExecNode for AggNode {
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

