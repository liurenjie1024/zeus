use std::vec::Vec;

use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use super::expression::EvalContext;
use super::expression::Expr;
use super::ServerContext;
use super::ExecNode;
use super::ExecContext;
use super::Block;
use util::errors::*;

pub struct FilterExecNode {
  filter: Option<Expr> ,
  input: Box<ExecNode>
}

impl ExecNode for FilterExecNode {
  fn open(&mut self, context: &mut ExecContext) -> Result<()> {
    self.input.open(context)
  }

  fn next(&mut self) -> Result<Block> {
    let input_block = self.input.next()?;
    let eval_context = EvalContext::default();

    match self.filter {
      Some(ref mut f) => {
        let masks = f.eval(&eval_context, &input_block)?;
        input_block.filter(&masks)
      }
      None => Ok(input_block)
    }
  }

  fn close(&mut self) -> Result<()> {
    self.input.close()
  }
}

impl FilterExecNode {
  pub fn new(plan_node: &PlanNode, _server_context: &ServerContext, mut children: Vec<Box<ExecNode>>)
    -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::FILTER_NODE,
      "Can't construct filter node from {:?}", plan_node.get_plan_node_type());
    ensure!(children.len() == 1,
      "Filter node should only have 1 child rather {}", children.len());
    ensure!(plan_node.get_filter_node().get_conditions().len() == 1,
      "Filter node condition can only support one condition");


    let input = children.pop().unwrap();

    let filters = plan_node.get_filter_node().get_conditions()
      .first()
      .map(Expr::new)
      .map_or(Ok(None), |t| t.map(Some))?;

    Ok(box FilterExecNode {
      filter: filters,
      input
    })
  }
}

#[cfg(test)]
mod tests {

  use std::default::Default;
  use std::convert::From;

  use storage::column::Column;
  use storage::column::vec_column_data::VecColumnData;
  use exec::tests::MemoryBlocks;
  use exec::Block;
  use exec::ExecNode;
  use exec::ExecContext;
  use super::FilterExecNode;
  use rpc::zeus_plan::{FilterNode, PlanNode, PlanNodeType};
  use rpc::zeus_expr::{Expression, ExpressionType, LiteralExpression};
  use rpc::zeus_meta::{ColumnType, ColumnValue};
  use server::ServerContext;

  fn create_memory_block() -> Box<ExecNode> {
    let column1 = Column::new_vec(ColumnType::BOOL, VecColumnData::from(vec![true, false]));
    let column2 = Column::new_vec(ColumnType::INT64, VecColumnData::from(vec![12i64, 14i64]));
    let block1 = vec![column1, column2];
    let block1 = Block::from(block1);

    let column3 = Column::new_vec(ColumnType::BOOL, VecColumnData::from(vec![false, true]));
    let column4 = Column::new_vec(ColumnType::INT64, VecColumnData::from(vec![100000i64, 54321i64]));
    let block2 = vec![column3, column4];
    let block2 = Block::from(block2);

    box MemoryBlocks {
      blocks: vec![block1, block2],
    }
  }

  fn create_filter_plan_node() -> PlanNode {
    let mut value = ColumnValue::new();
    value.set_bool_value(true);

    let mut literal_expr = LiteralExpression::new();
    literal_expr.set_field_type(ColumnType::BOOL);
    literal_expr.set_value(value);

    let mut expr = Expression::new();
    expr.set_expression_type(ExpressionType::LITERAL);
    expr.set_literal(literal_expr);


    let mut filter_node = FilterNode::new();
    filter_node.mut_conditions().push(expr);

    let mut plan_node = PlanNode::new();
    plan_node.set_node_id(1);
    plan_node.set_plan_node_type(PlanNodeType::FILTER_NODE);
    plan_node.set_filter_node(filter_node);

    plan_node
  }

  #[test]
  fn test_create_filter_exec_node() {
    let plan_node = create_filter_plan_node();
    let server_context = ServerContext::default();
    let children = vec![create_memory_block()];

    assert!(FilterExecNode::new(&plan_node, &server_context, children).is_ok());
  }

  #[test]
  fn test_create_filter_exec_node_wrong_type() {
    let mut plan_node = create_filter_plan_node();
    plan_node.set_plan_node_type(PlanNodeType::SCAN_NODE);

    let server_context = ServerContext::default();
    let children = vec![create_memory_block()];

    assert!(FilterExecNode::new(&plan_node, &server_context, children).is_err());
  }

  #[test]
  fn test_create_filter_exec_node_wrong_children() {
    let plan_node = create_filter_plan_node();

    let server_context = ServerContext::default();
    let children = vec![create_memory_block(), create_memory_block()];

    assert!(FilterExecNode::new(&plan_node, &server_context, children).is_err());
  }

  #[test]
  fn test_create_filter_exec_node_wrong_conditions() {
    let mut plan_node = create_filter_plan_node();
    let expr = plan_node.get_filter_node().get_conditions().first().unwrap().clone();
    plan_node.mut_filter_node().mut_conditions().push(expr);

    let server_context = ServerContext::default();
    let children = vec![create_memory_block(), create_memory_block()];

    assert!(FilterExecNode::new(&plan_node, &server_context, children).is_err());
  }

  #[test]
  fn test_execute_filter_exec_node() {
    let plan_node = create_filter_plan_node();
    let server_context = ServerContext::default();
    let children = vec![create_memory_block()];

    let mut filter_node = FilterExecNode::new(&plan_node, &server_context, children).unwrap();
    let mut exec_context = ExecContext::default();

    assert!(filter_node.open(&mut exec_context).is_ok());

    // First block
    let ret = filter_node.next();
    assert!(ret.is_ok());

    let ret = ret.unwrap();
    assert_eq!(2, ret.len());
    assert_eq!(2, ret.columns.len());
    assert!(!ret.eof);

    // Second block
    let ret = filter_node.next();
    assert!(ret.is_ok());

    let ret = ret.unwrap();
    assert_eq!(2, ret.len());
    assert_eq!(2, ret.columns.len());
    assert!(ret.eof);
  }
}
