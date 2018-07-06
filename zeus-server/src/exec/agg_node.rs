use std::vec::Vec;
use std::collections::HashMap;

use super::ServerContext;
use super::ExecNode;
use super::ExecContext;
use super::Block;
use super::expression::Expr;
use super::expression::agg_func::AggFunc;
use super::expression::agg_func;
use rpc::zeus_expr::AggFuncId;
use rpc::zeus_expr::Expression;
use rpc::zeus_expr::ExpressionType;
use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use rpc::zeus_meta::ColumnType;
use storage::column::Column;
use super::expression::EvalContext;
use storage::column::vec_column_data::Datum;
use storage::column::ColumnBuilder;
use util::errors::*;

struct AggExpr {
  agg_func_id: AggFuncId,
  args: Vec<Expr>,
  alias: String,
  field_type: ColumnType
}

pub struct AggExecNode {
  group_bys: Vec<Expr>,
  aggs: Vec<AggExpr>,
  data: HashMap<Vec<Datum>, Vec<Box<AggFunc>>>,
  input: Box<ExecNode>,
  executed: bool
}

impl ExecNode for AggExecNode {
  fn open(&mut self, context: &mut ExecContext) -> Result<()> {
    self.input.open(context)
  }

  fn next(&mut self) -> Result<Block> {
    if self.executed {
      return Err(ErrorKind::EOF.into())
    }

    self.executed = true;
    let eval_context = EvalContext::default();

    // Do calculation
    loop {
      let input = self.input.next()?;

      let group_by_block = self.group_bys.iter()
        .try_fold(Vec::new(), |mut columns, expr| -> Result<Vec<Column>> {
          columns.push(expr.eval(&eval_context, &input)?);
          Ok(columns)
        })?;

      let group_by_block = Block::new(group_by_block, false);

      let agg_blocks = self.aggs.iter()
        .try_fold(Vec::new(), |mut blocks, agg| -> Result<Vec<Block>> {
          blocks.push(agg.eval(&eval_context, &input)?);
          Ok(blocks)
        })?;

      let insert = |row_idx: usize, agg_funcs: &mut Vec<Box<AggFunc>>| -> Result<()> {
        agg_funcs.iter_mut()
          .zip(agg_blocks.iter())
          .try_for_each(|r| r.0.aggregate(r.1, row_idx))
      };

      group_by_block.iter()
        .enumerate()
        .try_for_each(|r| -> Result<()> {
          let idx = r.0;
          let row = r.1;

          let need_put = !self.data.contains_key(&row);

          if need_put {
            let mut agg_exprs = self.aggs.iter()
              .map(|agg| agg_func::func_of(agg.agg_func_id))
              .collect::<Vec<Box<AggFunc>>>();

            insert(idx, &mut agg_exprs)?;

            self.data.insert(row, agg_exprs);
          } else {
            insert(idx, self.data.get_mut(&row).unwrap())?;
          }

          Ok(())
        })?;

      if input.eof {
        break;
      }
    }

    // Output content

    let mut key_columns: Vec<Vec<Datum>> = vec![
      Vec::with_capacity(self.data.len());
      self.group_bys.len()];
    let mut value_columns: Vec<Vec<Datum>> = vec![
      Vec::with_capacity(self.data.len());
      self.aggs.len()];

    self.data.drain()
      .try_for_each(|entry| -> Result<()> {
        let (k, v) = entry;

        key_columns.iter_mut()
          .zip(k.into_iter())
          .for_each(|r| r.0.push(r.1));

        value_columns.iter_mut()
          .zip(v.into_iter())
          .try_for_each(|r| {
            let (c, mut agg) = r;
            c.push(agg.collect()?);
            Ok(())
          })
      })?;


    let mut key_columns: Vec<Column> = key_columns.into_iter()
      .zip(self.group_bys.iter())
      .map(|r| {
        let (column, expr) = r;
        ColumnBuilder::new_vec(expr.field_type(), column)
          .set_name(expr.alias_ref())
          .build()
      }).collect();

    let mut value_columns: Vec<Column> = value_columns.into_iter()
      .zip(self.aggs.iter())
      .map(|r| {
        let (column, expr) = r;
        ColumnBuilder::new_vec(expr.get_field_type(), column)
          .set_name(expr.alias_ref())
          .build()
      }).collect();

    let mut columns: Vec<Column> = Vec::with_capacity(key_columns.len() + value_columns.len());
    columns.append(&mut key_columns);
    columns.append(&mut value_columns);

    Ok(Block::new(columns, true))
  }

  fn close(&mut self) -> Result<()> {
    self.input.close()
  }
}

impl AggExpr {
  fn new(expr: &Expression) -> Result<AggExpr> {
    ensure!(expr.get_expression_type() == ExpressionType::AGG_FUNCTION,
      "Expression type should be AGG_FUNCTION");

    let agg_func = expr.get_agg_func();

    let args = agg_func.get_children().iter()
      .try_fold(Vec::new(), |mut res, expr| -> Result<Vec<Expr>> {
        res.push(Expr::new(expr)?);
        Ok(res)
      })?;

    Ok(AggExpr {
      agg_func_id: agg_func.get_func_id(),
      args,
      alias: expr.get_alias().to_string(),
      field_type: expr.get_field_type()
    })
  }

  fn eval(&self, ctx: &EvalContext, input: &Block) -> Result<Block> {
    let columns = self.args.iter()
      .try_fold(Vec::new(), |mut columns, arg| -> Result<Vec<Column>> {
        columns.push(arg.eval(ctx, input)?);
        Ok(columns)
      })?;
    Ok(Block::from(columns))
  }

  fn alias_ref(&self) -> &str {
    self.alias.as_str()
  }

  fn get_field_type(&self) -> ColumnType {
    self.field_type
  }
}

impl AggExecNode {
  pub fn new(plan_node: &PlanNode, _server_context: &ServerContext, mut children: Vec<Box<ExecNode>>)
    -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::AGGREGATE_NODE,
      "Can create aggregate node from {:?}", plan_node.get_plan_node_type());
    ensure!(children.len() == 1,
      "The children number of agg node should be 1 rather {}", children.len());

    let input = children.pop().unwrap();

    let agg_node = plan_node.get_agg_node();

    let group_bys = agg_node.get_group_by().iter()
      .try_fold(Vec::new(), |mut res, expr| -> Result<Vec<Expr>> {
        res.push(Expr::new(expr)?);
        Ok(res)
      })?;

    let aggs = agg_node.get_agg_func().iter()
      .try_fold(Vec::new(), |mut res, expr| -> Result<Vec<AggExpr>> {
        res.push(AggExpr::new(expr)?);
        Ok(res)
      })?;

    Ok(box AggExecNode {
      group_bys,
      aggs,
      data: HashMap::new(),
      input,
      executed: false
    })
  }
}

#[cfg(test)]
mod tests {
  use std::default::Default;
  use std::collections::HashSet;

  use storage::column::ColumnBuilder;
  use storage::column::vec_column_data::Datum;
  use exec::tests::MemoryBlocks;
  use exec::tests::ExpressionBuilder;
  use exec::Block;
  use exec::ExecNode;
  use exec::ExecContext;
  use super::AggExecNode;
  use rpc::zeus_plan::{PlanNode, PlanNodeType};
  use rpc::zeus_plan::AggregationNode;
  use rpc::zeus_expr::AggFuncId;
  use rpc::zeus_meta::ColumnType;
  use server::ServerContext;

  fn create_memory_block() -> Box<ExecNode> {
    let column1 = ColumnBuilder::new_vec(ColumnType::INT8, Datum::vec_of(vec![1i8, 2i8]))
      .set_name("a")
      .build();
    let column2 = ColumnBuilder::new_vec(ColumnType::INT64, Datum::vec_of(vec![3i64, 4i64]))
      .set_name("b")
      .build();
    let column3 = ColumnBuilder::new_vec(
      ColumnType::STRING, Datum::vec_of(vec!["love".to_string(), "rust".to_string()]))
      .set_name("c")
      .build();
    let block1 = vec![column1, column2, column3];
    let block1 = Block::from(block1);

    let column1 = ColumnBuilder::new_vec(ColumnType::INT8, Datum::vec_of(vec![5i8, 6i8]))
      .set_name("a")
      .build();
    let column2 = ColumnBuilder::new_vec(ColumnType::INT64, Datum::vec_of(vec![7i64, 8i64]))
      .set_name("b")
      .build();
    let column3 = ColumnBuilder::new_vec(
      ColumnType::STRING, Datum::vec_of(vec!["love".to_string(), "java".to_string()]))
      .set_name("c")
      .build();
    let block2 = vec![column1, column2, column3];
    let block2 = Block::from(block2);

    box MemoryBlocks {
      blocks: vec![block1, block2],
    }
  }

  fn create_agg_plan_node() -> PlanNode {
    // create expression of c
    let expr_c = ExpressionBuilder::new_column_ref("c", ColumnType::STRING)
      .build();



    // create expression sum(a)
    let expr_sum_a = {
      let expr_a = ExpressionBuilder::new_column_ref("a", ColumnType::INT8)
        .build();

      let expr = ExpressionBuilder::new_agg_func("sum(a)", ColumnType::INT8, AggFuncId::SUM_INT32)
        .add_children(expr_a)
        .build();

      expr
    };


    // create expression sum(b)
    let expr_sum_b = {
      let expr_b = ExpressionBuilder::new_column_ref("b", ColumnType::INT64)
        .build();

      let expr = ExpressionBuilder::new_agg_func("sum(b)", ColumnType::INT64, AggFuncId::SUM_INT32)
        .add_children(expr_b)
        .build();

      expr
    };


    let mut agg_node = AggregationNode::new();
    agg_node.mut_group_by().push(expr_c);
    agg_node.mut_agg_func().push(expr_sum_a);
    agg_node.mut_agg_func().push(expr_sum_b);

    let mut plan_node = PlanNode::new();
    plan_node.set_node_id(1);
    plan_node.set_plan_node_type(PlanNodeType::AGGREGATE_NODE);
    plan_node.set_agg_node(agg_node);

    plan_node
  }

  #[test]
  fn test_create_agg_exec_node() {
    let plan_node = create_agg_plan_node();
    let server_context = ServerContext::default();
    let children = vec![create_memory_block()];

    assert!(AggExecNode::new(&plan_node, &server_context, children).is_ok());
  }

  #[test]
  fn test_create_agg_exec_node_wrong_type() {
    let mut plan_node = create_agg_plan_node();
    plan_node.set_plan_node_type(PlanNodeType::SCAN_NODE);

    let server_context = ServerContext::default();
    let children = vec![create_memory_block()];

    assert!(AggExecNode::new(&plan_node, &server_context, children).is_err());
  }

  #[test]
  fn test_create_agg_exec_node_wrong_children() {
    let plan_node = create_agg_plan_node();

    let server_context = ServerContext::default();
    let children = vec![create_memory_block(), create_memory_block()];

    assert!(AggExecNode::new(&plan_node, &server_context, children).is_err());
  }

  #[test]
  fn test_execute_agg_exec_node() {
    let plan_node = create_agg_plan_node();
    let server_context = ServerContext::default();
    let children = vec![create_memory_block()];

    let mut agg_node = AggExecNode::new(&plan_node, &server_context, children).unwrap();
    let mut exec_context = ExecContext::default();

    assert!(agg_node.open(&mut exec_context).is_ok());

    // First block
    let ret = agg_node.next();
    assert!(ret.is_ok());

    let ret = ret.unwrap();
    assert_eq!(3, ret.len());
    assert_eq!(3, ret.columns.len());
    assert_eq!(Some("c"), ret.columns[0].name());
    assert_eq!(Some("sum(a)"), ret.columns[1].name());
    assert_eq!(Some("sum(b)"), ret.columns[2].name());

    let mut expected_result = HashSet::new();
    vec![
      vec![Datum::UTF8("love".to_string()), Datum::Int8(6i8), Datum::Int64(10i64)],
      vec![Datum::UTF8("rust".to_string()), Datum::Int8(2i8), Datum::Int64(4i64)],
      vec![Datum::UTF8("java".to_string()), Datum::Int8(6i8), Datum::Int64(8i64)],
    ].into_iter().for_each(|r| {
      expected_result.insert(r);
    });

    let mut result = HashSet::new();
    ret.iter()
      .for_each(|r| {
        result.insert(r.clone());
      });

    assert_eq!(expected_result, result);
    assert!(ret.eof);


    // Second block
    let ret = agg_node.next();
    assert!(ret.is_err());
  }
}
