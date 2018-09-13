use std::vec::Vec;
use std::alloc::alloc;
use std::ptr;

use super::expression::AggregationExpr;
use super::exec::ExecNode;
use super::exec::ExecContext;
use super::expression::AggregationFunction;
use super::block::Block;

use util::errors::*;

struct ExpressionWithData {
  expression: AggregationExpr,
  function: Box<AggregationFunction>,
  data: *mut u8
}

struct SimpleAggregationExecNode {
  expression_with_data: Vec<ExpressionWithData>,
  child: Box<ExecNode>
}

impl ExecNode for SimpleAggregationExecNode {
  fn open(&mut self, ctx: &ExecContext) -> Result<()> {
    self.child.open(ctx)
  }

  fn next(&mut self) -> Result<Block> {
    loop {
      let input = self.child.next()?;

      self.expression_with_data.iter()
        .for_each(|d| {
          let expr_array = d.expression

        })
    }
  }

  fn close(&mut self) -> Result<()> {
    unimplemented!()
  }
}

