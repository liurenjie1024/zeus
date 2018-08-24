pub mod table_scan_node;
pub mod agg_node;
pub mod filter_node;
pub mod project_node;
pub mod limit_node;
pub mod topn_node;
pub mod expression;
pub mod get_row_num_node;


use std::boxed::Box;
use std::vec::Vec;
use std::default::Default;

use futures::sync::oneshot::Sender;

use storage::column::Column;
use util::errors::*;
use rpc::zeus_data::RowResult;
use rpc::zeus_data::QueryRequest;
use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use scheduler::Task;
use storage::column::ColumnValueIter;
use storage::column::vec_column_data::Datum;
use server::ServerContext;
use server::data_service::Rows;
use self::table_scan_node::TableScanNode;
use self::limit_node::LimitExecNode;
use self::filter_node::FilterExecNode;
use self::project_node::ProjectExecNode;
use self::agg_node::HashAggExecNode;
use self::topn_node::TopNExecNode;
use self::get_row_num_node::GetRowNumExecNode;

#[derive(Debug)]
pub struct Block {
  pub columns: Vec<Column>,
  pub eof: bool,
}

impl Block {
  pub fn new(columns: Vec<Column>, eof: bool) -> Block {
    Block {
      columns,
      eof
    }
  }

  pub fn from(columns: Vec<Column>) -> Block {
    //TODO: Check that columns are equal length
    Block {
      columns,
      eof: true,
    }
  }

  pub fn len(&self) -> usize {
    self.columns.first()
      .map_or(0usize, |c| c.size())
  }

  pub fn take(&self, num: usize) -> Block {
    Block {
      columns: self.columns.iter().map(|x| x.take(num)).collect(),
      eof: self.eof
    }
  }

  pub fn is_empty(&self) -> bool {
    self.columns.is_empty()
  }

  pub fn column_by_name(&self, name: &str) -> Option<Column> {
    self.columns.iter()
      .find(|x| x.name() == Some(name))
      .map(|c| c.clone())
  }

  pub fn filter(&self, masks: &Column) -> Result<Block> {
    let columns = self.columns.iter()
        .try_fold(Vec::new(), |mut res, input| -> Result<Vec<Column>> {
          let filtered_column = input.filter(masks)?;
          res.push(filtered_column);
          Ok(res)
        })?;

    Ok(Block {
      columns,
      eof: self.eof
    })
  }

  pub fn merge(&mut self, other: Column) -> Result<()> {
    // TODO: Check that all columns are equal length
    self.columns.push(other);
    Ok(())
  }

  pub fn is_same_type(&self, other: &Block) -> bool {
    if self.columns.len() != other.columns.len() {
      return false;
    }

    self.columns.iter().zip(other.columns.iter())
      .all(|t| t.0.field_type() == t.1.field_type())
  }

  pub fn iter(&self) -> BlockIterator {
    BlockIterator {
      block: self,
      idx: 0
    }
  }

  pub fn is_eof(&self) -> bool {
    self.eof
  }

  pub fn columns_slice(&self) -> &[Column] {
    &self.columns
  }
}



impl Default for Block {
  fn default() -> Self {
    Block {
      columns: Vec::new(),
      eof: true
    }
  }
}

pub struct BlockIterator<'a> {
  block: &'a Block,
  idx: usize
}

impl<'a> Iterator for BlockIterator<'a> {
  type Item = Vec<Datum>;

  fn next(&mut self) -> Option<Vec<Datum>> {
    let ret = self.block.columns.iter()
      .map(|c| c.get(self.idx))
      .fold(Some(Vec::new()), |row, d| {
        match (row, d) {
          (Some(mut v), Some(d)) => {
            v.push(d);
            Some(v)
          },
          _ => None
        }
      });

    self.idx += 1;
    ret
  }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecPhase {
  UnInited,
  Opened,
  Executed,
  Closed,
}

pub struct ExecContext {}

impl Default for ExecContext {
  fn default() -> Self {
    ExecContext {}
  }
}

pub trait ExecNode {
  fn open(
    &mut self,
    context: &mut ExecContext,
  ) -> Result<()>;
  fn next(&mut self) -> Result<Block>;
  fn close(&mut self) -> Result<()>;
}

pub struct DAGExecutor {
  root: Box<ExecNode>,
  sender: Sender<Result<Rows>>,
}


impl PlanNode {
  pub fn to_exec_node(&self, server_context: &ServerContext) -> Result<Box<ExecNode>> {
    let children = self.get_children()
      .iter()
      .try_fold(Vec::new(), |mut res, plan_node| -> Result<Vec<Box<ExecNode>>> {
        res.push(plan_node.to_exec_node(server_context)?);
        Ok(res)
      })?;
    match self.get_plan_node_type() {
      PlanNodeType::SCAN_NODE => TableScanNode::new(self.get_scan_node(), server_context, children),
      PlanNodeType::LIMIT_NODE => LimitExecNode::new(&self, server_context, children),
      PlanNodeType::FILTER_NODE => FilterExecNode::new(&self, server_context, children),
      PlanNodeType::PROJECT_NODE => ProjectExecNode::new(&self, server_context, children),
      PlanNodeType::AGGREGATE_NODE => HashAggExecNode::new(&self, server_context, children),
      PlanNodeType::TOPN_NODE => TopNExecNode::new(&self, server_context, children),
      PlanNodeType::GET_ROW_NUM_NODE => GetRowNumExecNode::new(&self, server_context, children)
    }
  }
}

impl DAGExecutor {
  pub fn task(
    query_request: QueryRequest,
    sender: Sender<Result<Rows>>,
    server_context: ServerContext,
  ) -> Task
  {
    let task_body = box move || {
      let root_result = query_request.get_plan().get_root().to_exec_node(&server_context);

      match root_result {
        Ok(root) => {
          let dag = DAGExecutor {
            root,
            sender,
          };
          dag.run();
        },
        Err(e) => {
          sender.send(Err(e))
            .err()
            .iter()
            .for_each(|e| error!("Failed to send error: {:?}", e));
        },
      }
    };

    Task::new(task_body)
  }
}

impl DAGExecutor {
  fn run(mut self) {
    let result = self.query();
    match self.sender.send(result) {
      Ok(_) => info!("succeeded to send query result!"),
      Err(_) => error!("Failed to send query result!"),
    }
  }

  fn query(&mut self) -> Result<Rows> {
    let mut context = ExecContext {};

    self.root.open(&mut context)?;

    let mut rows = Rows::new();

    loop {
      let block = self.root.next()?;

      if !block.is_empty() {
        let mut column_iterators: Vec<ColumnValueIter> =
          block.columns.iter().map(|c| c.column_value_iter()).collect();

        loop {
          let mut incomplete = 0usize;
          let mut row = RowResult::new();
          for column_iter in &mut column_iterators {
            match column_iter.next() {
              Some(t) => row.columns.push(t),
              None => {
                incomplete += 1;
              },
            }
          }

          if 0 == incomplete {
            rows.push(row);
          } else {
            if incomplete < column_iterators.len() {
              warn!("Block incomplete, there are {} columns incomplete.", incomplete);
            }
            break;
          }
        }
      }

      if block.eof {
        break;
      }
    }

    Ok(rows)
  }
}

#[cfg(test)]
mod tests {
  use std::vec::Vec;

  use futures::sync::oneshot::channel;
  use futures::Future;
  use futures::Async;
  use protobuf::RepeatedField;

  use super::ExecNode;
  use super::Block;
  use super::ExecContext;
  use super::DAGExecutor;
  use rpc::zeus_data::RowResult;
  use rpc::zeus_meta::ColumnValue;
  use rpc::zeus_meta::ColumnType;
  use rpc::zeus_expr::Expression;
  use rpc::zeus_expr::ExpressionType;
  use rpc::zeus_expr::ColumnRef;
  use rpc::zeus_expr::ScalarFuncId;
  use rpc::zeus_expr::AggFuncId;
  use rpc::zeus_expr::ScalarFunction;
  use rpc::zeus_expr::AggFunction;
  use rpc::zeus_expr::LiteralExpression;
  use storage::column::Column;
  use storage::column::vec_column_data::VecColumnData;
  use util::errors::*;

  pub struct MemoryBlocks {
    pub blocks: Vec<Block>,
  }

  impl ExecNode for MemoryBlocks {
    fn open(
      &mut self,
      _context: &mut ExecContext,
    ) -> Result<()>
    {
      Ok(())
    }

    fn next(&mut self) -> Result<Block> {
      assert!(!self.blocks.is_empty());

      let mut block = self.blocks.pop().unwrap();
      block.eof = self.blocks.is_empty();
      Ok(block)
    }

    fn close(&mut self) -> Result<()> {
      Ok(())
    }
  }

  #[derive(Default)]
  pub struct ExpressionBuilder {
    expression_type: ExpressionType,
    alias: String,
    field_type: ColumnType,

    // column ref
    name: String,

    // literal
    value: ColumnValue,

    // Scalar function
    scalar_func_id: ScalarFuncId,

    // agg function
    agg_func_id: AggFuncId,

    // children
    children: Vec<Expression>
  }

  impl ExpressionBuilder {
    pub fn new_column_ref<S: ToString>(name: S, field_type: ColumnType) -> ExpressionBuilder {
      let mut builder = ExpressionBuilder::default();
      builder.expression_type = ExpressionType::COLUMN_REF;
      builder.alias = name.to_string();
      builder.field_type = field_type;
      builder.name = name.to_string();

      builder
    }

    #[allow(dead_code)]
    pub fn new_literal<S: ToString>(alias: S,
                                    field_type: ColumnType,
                                    column_value: ColumnValue) -> ExpressionBuilder {
      let mut builder = ExpressionBuilder::default();
      builder.expression_type = ExpressionType::LITERAL;
      builder.alias = alias.to_string();
      builder.field_type = field_type;
      builder.value = column_value;

      builder
    }

    #[allow(dead_code)]
    pub fn new_scalar_func<S: ToString>(alias: S,
                                        field_type: ColumnType,
                                        scalar_func_id: ScalarFuncId) -> ExpressionBuilder {
      let mut builder = ExpressionBuilder::default();
      builder.expression_type = ExpressionType::SCALAR_FUNCTION;
      builder.alias = alias.to_string();
      builder.field_type = field_type;
      builder.scalar_func_id = scalar_func_id;

      builder
    }

    pub fn new_agg_func<S: ToString>(alias: S,
                                     field_type: ColumnType,
                                     agg_func_id: AggFuncId) -> ExpressionBuilder {
      let mut builder = ExpressionBuilder::default();
      builder.expression_type = ExpressionType::AGG_FUNCTION;
      builder.alias = alias.to_string();
      builder.field_type = field_type;
      builder.agg_func_id = agg_func_id;

      builder
    }

    // column ref
    #[allow(dead_code)]
    pub fn set_name<S: ToString>(mut self, alias: S) -> ExpressionBuilder {
      self.alias = alias.to_string();
      self
    }

    pub fn add_children(mut self, child: Expression) -> ExpressionBuilder {
      self.children.push(child);
      self
    }

    pub fn build(self) -> Expression {
      let mut expr = Expression::new();
      expr.set_expression_type(self.expression_type);
      expr.set_alias(self.alias);
      expr.set_field_type(self.field_type);

      match self.expression_type {
        ExpressionType::COLUMN_REF => {
          let mut column_ref = ColumnRef::new();
          column_ref.set_name(self.name);

          expr.set_column(column_ref);
        }
        ExpressionType::LITERAL => {
          let mut literal = LiteralExpression::new();
          literal.set_value(self.value);

          expr.set_literal(literal);
        }
        ExpressionType::SCALAR_FUNCTION => {
          let mut scalar_func = ScalarFunction::new();
          scalar_func.set_func_id(self.scalar_func_id);
          scalar_func.set_children(RepeatedField::from_vec(self.children));

          expr.set_scalar_func(scalar_func);
        }
        ExpressionType::AGG_FUNCTION => {
          let mut agg_func = AggFunction::new();
          agg_func.set_func_id(self.agg_func_id);
          agg_func.set_children(RepeatedField::from_vec(self.children));

          expr.set_agg_func(agg_func);
        }
      }

      expr
    }
  }



  #[test]
  fn test_run() {
    let column1 = Column::new_vec(ColumnType::BOOL, VecColumnData::from(vec![true, false]));
    let column2 = Column::new_vec(ColumnType::INT64, VecColumnData::from(vec![12i64, 14i64]));
    let block1 = vec![column1, column2];
    let block1 = Block::from(block1);

    let column3 = Column::new_vec(ColumnType::BOOL, VecColumnData::from(vec![false, true]));
    let column4 = Column::new_vec(ColumnType::INT64, VecColumnData::from(vec![100000i64, 54321i64]));
    let block2 = vec![column3, column4];
    let block2 = Block::from(block2);

    let mem_blocks = box MemoryBlocks {
      blocks: vec![block1, block2],
    };

    let (sender, mut receiver) = channel();
    let dag = DAGExecutor {
      root: mem_blocks,
      sender,
    };

    dag.run();
    let result = receiver.poll().unwrap();

    let expected_rows = vec![
      to_row_result(false, 100000i64),
      to_row_result(true, 54321i64),
      to_row_result(true, 12i64),
      to_row_result(false, 14i64),
    ];
    match result {
      Async::Ready(t) => assert_eq!(expected_rows, t.ok().unwrap()),
      Async::NotReady => assert!(false, "Result should be ready!"),
    }
  }

  fn to_row_result(
    v1: bool,
    v2: i64,
  ) -> RowResult
  {
    let mut r = RowResult::new();

    let mut c1 = ColumnValue::new();
    c1.set_bool_value(v1);

    let mut c2 = ColumnValue::new();
    c2.set_i64_value(v2);

    r.mut_columns().push(c1);
    r.mut_columns().push(c2);

    r
  }
}
