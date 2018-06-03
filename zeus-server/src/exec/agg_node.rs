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
  _agg_func_id: AggFuncId,
  _args: Vec<Expr>,
  alias: String,
  field_type: ColumnType
}

pub struct AggExecNode {
  _group_bys: Vec<Expr>,
  _aggs: Vec<AggExpr>,
  _data: HashMap<Vec<Datum>, Vec<Box<AggFunc>>>,
  input: Box<ExecNode>,
  executed: bool
}

impl ExecNode for AggExecNode {
  fn open(&mut self, context: &mut ExecContext) -> Result<()> {
    self.input.open(context)
  }

  fn next(&mut self) -> Result<Block> {
    if self.executed {
      return Ok(Block::default())
    }

    self.executed = true;
    let eval_context = EvalContext::default();

    // Do calculation
    loop {
      let input = self.next()?;

      let group_by_block = self._group_bys.iter()
        .try_fold(Vec::new(), |mut columns, expr| -> Result<Vec<Column>> {
          columns.push(expr.eval(&eval_context, &input)?);
          Ok(columns)
        })?;

      let group_by_block = Block::new(group_by_block, false);
      let agg_blocks = self._aggs.iter()
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

          let need_put = self._data.contains_key(&row);

          if need_put {
            let mut agg_exprs = self._aggs.iter()
              .map(|agg| agg_func::func_of(agg._agg_func_id))
              .collect::<Vec<Box<AggFunc>>>();

            insert(idx, &mut agg_exprs)?;

            self._data.insert(row, agg_exprs);
          } else {
            insert(idx, self._data.get_mut(&row).unwrap())?;
          }

          Ok(())
        })?;

      if input.eof {
        break;
      }
    }

    // Output content

    let mut key_columns: Vec<Vec<Datum>> = vec![
      Vec::with_capacity(self._data.len());
      self._group_bys.len()];
    let mut value_columns: Vec<Vec<Datum>> = vec![
      Vec::with_capacity(self._data.len());
      self._aggs.len()];

    self._data.drain()
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
      .zip(self._group_bys.iter())
      .map(|r| {
        let (column, expr) = r;
        ColumnBuilder::new_vec(expr.field_type(), column)
          .set_name(expr.alias_ref())
          .build()
      }).collect();

    let mut value_columns: Vec<Column> = value_columns.into_iter()
      .zip(self._aggs.iter())
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
      _agg_func_id: agg_func.get_func_id(),
      _args: args,
      alias: expr.get_alias().to_string(),
      field_type: expr.get_field_type()
    })
  }

  fn eval(&self, ctx: &EvalContext, input: &Block) -> Result<Block> {
    let columns = self._args.iter()
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
      _group_bys: group_bys,
      _aggs: aggs,
      _data: HashMap::new(),
      input,
      executed: false
    })
  }
}

