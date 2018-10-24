use std::vec::Vec;
use std::alloc::alloc;
use std::ptr;

use arrow::datatypes::Field;
use arrow::datatypes::Schema;
use arrow::array::ArrayRef;

use super::expression::AggregationExpr;
use super::exec::ExecNode;
use super::exec::ExecContext;
use super::expression::AggregationFunction;
use super::block::Block;
use super::expression::EvalContext;
use exec2::column::to_column_builder;

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
    let eval_context = EvalContext::default();

    loop {
      let input = self.child.next()?;

      if !input.is_empty() {
        self.expression_with_data.iter()
          .for_each(|d| {
            let expr_array = d.expression.args_slice()
              .first()
              .ok_or_else(|| "Children can't be empty!")
              .and_then(|e| e.eval(&eval_context, &input))
              .and_then(|b| b.get_column(0usize))?;

            unsafe {
              d.function.aggregate_all(d.data, &*expr_array);
            }
          });
      }

      if input.eof() {
        break;
      }
    }

    let fields = self.expression_with_data.iter()
      .map(|e| e.expression.to_field())
      .collect::<Vec<Field>>();

  }

  fn close(&mut self) -> Result<()> {
    unimplemented!()
  }
}

impl ExpressionWithData {
  fn to_array(&mut self) -> Result<ArrayRef> {
    let mut column_builder = to_column_builder(
      self.expression.column_type().to_arrow_data_type())?;

    unsafe {
      self.function.collect(self.data, &mut *column_builder);
      Ok(column_builder.into_array())
    }
  }
}

