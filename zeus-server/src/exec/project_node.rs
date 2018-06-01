use std::vec::Vec;

use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use storage::column::Column;
use super::expression::EvalContext;
use super::ServerContext;
use super::ExecNode;
use super::expression::Expr;
use super::ExecContext;
use super::Block;
use util::errors::*;

pub struct ProjectExecNode {
  mappers: Vec<Expr>,
  output_names: Vec<Option<String>>,
  input: Box<ExecNode>
}

impl ExecNode for ProjectExecNode {
  fn open(&mut self, context: &mut ExecContext) -> Result<()> {
    self.input.open(context)
  }

  fn next(&mut self) -> Result<Block> {
    let next_input_block = self.input.next()?;
    let eval_context = EvalContext::default();

    let mut columns = self.mappers.iter_mut()
      .try_fold(Vec::new(), |mut columns, expr| -> Result<Vec<Column>> {
        let mut column = expr.eval(&eval_context, &next_input_block)?;
        columns.push(column);
        Ok(columns)
      })?;

    columns.iter_mut()
      .zip(self.output_names.iter())
      .for_each(|t| match t.1 {
        Some(name) => t.0.set_name(name),
        None => ()
      });

    Ok(Block::new(columns, next_input_block.eof))
  }

  fn close(&mut self) -> Result<()> {
    self.input.close()
  }
}

impl ProjectExecNode {
  pub fn new(plan_node: &PlanNode, _server_context: &ServerContext, mut children: Vec<Box<ExecNode>>)
    -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::PROJECT_NODE,
      "Can't create project node from {:?}", plan_node.get_plan_node_type());
    ensure!(children.len() == 1,
      "Project node's children size should be 1 rather {}", children.len());

    let input = children.pop().unwrap();

    let output_names = plan_node.get_project_node()
      .get_items()
      .iter()
      .map(|item| {
        match item.get_alias() {
          "" => None,
          x  => Some(x.to_string())
        }
      }).collect();

    let mappers = plan_node.get_project_node().get_items()
      .iter()
      .try_fold(Vec::new(), |mut res, item| -> Result<Vec<Expr>> {
        res.push(Expr::new(item.get_expression())?);
        Ok(res)
      })?;



    Ok(box ProjectExecNode {
      mappers,
      output_names,
      input
    })
  }
}

#[cfg(test)]
mod tests {
  use std::default::Default;
  use std::vec::Vec;

  use storage::column::ColumnBuilder;
  use storage::column::vec_column_data::Datum;
  use exec::tests::MemoryBlocks;
  use exec::Block;
  use exec::ExecNode;
  use exec::ExecContext;
  use super::ProjectExecNode;
  use rpc::zeus_plan::ProjectNode_ProjectItem;
  use rpc::zeus_plan::{ProjectNode, PlanNode, PlanNodeType};
  use rpc::zeus_expr::{Expression, ExpressionType, LiteralExpression, ScalarFunction, ScalarFuncId,
    ColumnRef};
  use rpc::zeus_meta::{ColumnType, ColumnValue};
  use server::ServerContext;

  fn create_memory_block() -> Box<ExecNode> {
    let column1 = ColumnBuilder::new_vec(ColumnType::BOOL, Datum::vec_of(vec![true, true]))
      .set_name("b")
      .build();
    let column2 = ColumnBuilder::new_vec(ColumnType::INT64, Datum::vec_of(vec![16i64, 10000i64]))
      .set_name("a")
      .build();
    let block1 = vec![column1, column2];
    let block1 = Block::from(block1);

    let column3 = ColumnBuilder::new_vec(ColumnType::BOOL, Datum::vec_of(vec![false, false]))
      .set_name("b")
      .build();
    let column4 = ColumnBuilder::new_vec(ColumnType::INT64, Datum::vec_of(vec![5432i64, 12i64]))
      .set_name("a")
      .build();
    let block2 = vec![column3, column4];
    let block2 = Block::from(block2);

    box MemoryBlocks {
      blocks: vec![block1, block2],
    }
  }

  fn create_project_plan_node() -> PlanNode {
    // create expression of a > 18
    let expr_a_gt_18 = {

      // create column expression a
      let column_expr_a = {
        let mut tmp = ColumnRef::new();
        tmp.set_name("a".to_string());

        let mut expr = Expression::new();
        expr.set_expression_type(ExpressionType::COLUMN_REF);
        expr.set_column(tmp);

        expr
      };


      // create literal expression 18
      let const_expr = {
        let mut value = ColumnValue::new();
        value.set_i64_value(18i64);

        let mut literal_expr = LiteralExpression::new();
        literal_expr.set_field_type(ColumnType::INT64);
        literal_expr.set_value(value);

        let mut expr = Expression::new();
        expr.set_expression_type(ExpressionType::LITERAL);
        expr.set_literal(literal_expr);

        expr
      };


      // create scala func expr
      let mut scalar_func = ScalarFunction::new();
      scalar_func.set_func_id(ScalarFuncId::GT_I64);
      scalar_func.mut_children().push(column_expr_a);
      scalar_func.mut_children().push(const_expr);


      let mut expr = Expression::new();
      expr.set_expression_type(ExpressionType::SCALAR_FUNCTION);
      expr.set_scalar_func(scalar_func);


      let mut item = ProjectNode_ProjectItem::new();
      item.set_expression(expr);
      item.set_alias("x1".to_string());

      item
    };


    // create expression b
    let expr_b = {
      let mut tmp = ColumnRef::new();
      tmp.set_name("b".to_string());

      let mut expr = Expression::new();
      expr.set_expression_type(ExpressionType::COLUMN_REF);
      expr.set_column(tmp);

      let mut item = ProjectNode_ProjectItem::new();
      item.set_expression(expr);

      item
    };


    let mut project_node = ProjectNode::new();
    project_node.mut_items().push(expr_a_gt_18);
    project_node.mut_items().push(expr_b);

    let mut plan_node = PlanNode::new();
    plan_node.set_node_id(1);
    plan_node.set_plan_node_type(PlanNodeType::PROJECT_NODE);
    plan_node.set_project_node(project_node);

    plan_node
  }

  #[test]
  fn test_create_project_exec_node() {
    let plan_node = create_project_plan_node();
    let server_context = ServerContext::default();
    let children = vec![create_memory_block()];

    assert!(ProjectExecNode::new(&plan_node, &server_context, children).is_ok());
  }

  #[test]
  fn test_create_project_exec_node_wrong_type() {
    let mut plan_node = create_project_plan_node();
    plan_node.set_plan_node_type(PlanNodeType::SCAN_NODE);

    let server_context = ServerContext::default();
    let children = vec![create_memory_block()];

    assert!(ProjectExecNode::new(&plan_node, &server_context, children).is_err());
  }

  #[test]
  fn test_create_project_exec_node_wrong_children() {
    let plan_node = create_project_plan_node();

    let server_context = ServerContext::default();
    let children = vec![create_memory_block(), create_memory_block()];

    assert!(ProjectExecNode::new(&plan_node, &server_context, children).is_err());
  }

  #[test]
  fn test_execute_project_exec_node() {
    let plan_node = create_project_plan_node();
    let server_context = ServerContext::default();
    let children = vec![create_memory_block()];

    let mut project_node = ProjectExecNode::new(&plan_node, &server_context, children).unwrap();
    let mut exec_context = ExecContext::default();

    assert!(project_node.open(&mut exec_context).is_ok());

    // First block
    let ret = project_node.next();
    assert!(ret.is_ok());

    let ret = ret.unwrap();
    assert_eq!(2, ret.len());
    assert_eq!(2, ret.columns.len());
    assert_eq!(vec![Datum::Bool(true), Datum::Bool(false)], ret.columns[0].iter().collect::<Vec<Datum>>());
    assert_eq!(Some("x1"), ret.columns[0].name());
    assert_eq!(vec![Datum::Bool(false), Datum::Bool(false)], ret.columns[1].iter().collect::<Vec<Datum>>());
    assert_eq!(Some("b"), ret.columns[1].name());
    assert!(!ret.eof);

    // Second block
    let ret = project_node.next();
    assert!(ret.is_ok());

    let ret = ret.unwrap();
    assert_eq!(2, ret.len());
    assert_eq!(2, ret.columns.len());
    assert_eq!(vec![Datum::Bool(false), Datum::Bool(true)], ret.columns[0].iter().collect::<Vec<Datum>>());
    assert_eq!(Some("x1"), ret.columns[0].name());
    assert_eq!(vec![Datum::Bool(true), Datum::Bool(true)], ret.columns[1].iter().collect::<Vec<Datum>>());
    assert_eq!(Some("b"), ret.columns[1].name());
    assert!(ret.eof);
  }
}