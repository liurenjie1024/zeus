pub mod table_scan_node;
pub mod agg_node;
pub mod filter_node;
pub mod project_node;
pub mod limit_node;
pub mod topn_node;
pub mod expression;


use std::boxed::Box;
use std::vec::Vec;

use futures::sync::oneshot::Sender;

use storage::column::Column;
use util::errors::*;
use rpc::zeus_data::RowResult;
use rpc::zeus_data::QueryRequest;
use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use scheduler::Task;
use storage::column::ColumnValueIter;
use server::ServerContext;
use server::data_service::Rows;
use self::table_scan_node::TableScanNode;
use self::limit_node::LimitExecNode;
use self::filter_node::FilterNode;
use self::project_node::ProjectNode;
use self::agg_node::AggNode;
use self::topn_node::TopNNode;

pub struct ColumnWithInfo {
  pub name: String,
  pub id: Option<i32>,
  pub column: Column,
}

impl ColumnWithInfo {
  pub fn from(column: Column) -> ColumnWithInfo {
    ColumnWithInfo {
      name: "".to_string(),
      id: None,
      column,
    }
  }

  pub fn take(&self, num: usize) -> ColumnWithInfo {
    ColumnWithInfo {
      name: self.name.clone(),
      id: self.id,
      column: self.column.take(num)
    }
  }
}

pub struct Block {
  pub columns: Vec<ColumnWithInfo>,
  pub eof: bool,
}

impl Block {
  pub fn from(columns: Vec<ColumnWithInfo>) -> Block {
    //TODO: Check that columns are equal length
    Block {
      columns,
      eof: true,
    }
  }

  pub fn empty_block() -> Block {
    Block {
      columns: Vec::new(),
      eof: true
    }
  }

  pub fn len(&self) -> usize {
    self.columns.first()
      .map_or(0usize, |c| c.column.size())
  }

  pub fn take(&self, num: usize) -> Block {
    Block {
      columns: self.columns.iter().map(|x| x.take(num)).collect(),
      eof: self.eof
    }
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

pub trait ExecNode: Send + 'static {
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

unsafe impl Send for DAGExecutor {}


impl PlanNode {
  pub fn to(&self, server_context: &ServerContext) -> Result<Box<ExecNode>> {
    let children = self.get_children()
      .iter()
      .try_fold(Vec::new(), |mut res, plan_node| -> Result<Vec<Box<ExecNode>>> {
        res.push(plan_node.to(server_context)?);
        Ok(res)
      })?;
    match self.get_plan_node_type() {
      PlanNodeType::SCAN_NODE => TableScanNode::new(self.get_scan_node(), server_context, children),
      PlanNodeType::LIMIT_NODE => LimitExecNode::new(&self, server_context, children),
      PlanNodeType::FILTER_NODE => FilterNode::new(&self, server_context, children),
      PlanNodeType::PROJECT_NODE => ProjectNode::new(&self, server_context, children),
      PlanNodeType::AGGREGATE_NODE => AggNode::new(&self, server_context, children),
      PlanNodeType::TOPN_NODE => TopNNode::new(&self, server_context, children)
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
      let root_result = query_request.get_plan().get_root().to(&server_context);

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

      let mut column_iterators: Vec<ColumnValueIter> =
        block.columns.iter().map(|c| c.column.iter()).collect();

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

  use super::ExecNode;
  use super::Block;
  use super::ExecContext;
  use super::ColumnWithInfo;
  use super::DAGExecutor;
  use rpc::zeus_data::RowResult;
  use rpc::zeus_meta::ColumnValue;
  use rpc::zeus_meta::ColumnType;
  use storage::column::Column;
  use storage::column::column_data::ColumnData;
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

  #[test]
  fn test_run() {
    let column1 = Column::new(ColumnType::BOOL, ColumnData::from(vec![true, false]));
    let column2 = Column::new(ColumnType::INT64, ColumnData::from(vec![12i64, 14i64]));
    let block1 = vec![ColumnWithInfo::from(column1), ColumnWithInfo::from(column2)];
    let block1 = Block::from(block1);

    let column3 = Column::new(ColumnType::BOOL, ColumnData::from(vec![false, true]));
    let column4 = Column::new(ColumnType::INT64, ColumnData::from(vec![100000i64, 54321i64]));
    let block2 = vec![ColumnWithInfo::from(column3), ColumnWithInfo::from(column4)];
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
