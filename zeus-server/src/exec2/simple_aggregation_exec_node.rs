use std::vec::Vec;

use super::exec::ExecNode;
use super::exec::ExecContext;
use super::expression::AggregationFunction;
use super::block::Block;

use util::errors::*;

struct SimpleAggregationExecNode {
  aggregation_functions: Vec<Box<AggregationFunction>>,
  data: Vec<*mut u8>,
}

impl ExecNode for SimpleAggregationExecNode {
  fn open(&mut self, ctx: &ExecContext) -> Result<()> {
    unimplemented!()
  }

  fn next(&mut self) -> Result<Block> {
    unimplemented!()
  }

  fn close(&mut self) -> Result<()> {
    unimplemented!()
  }
}

