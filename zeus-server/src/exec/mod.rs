pub mod table_scan_node;

use std::boxed::Box;
use std::vec::Vec;

use futures::sync::oneshot::Sender;

use storage::column::Column;
use util::cow_ptr::CowPtr;
use util::errors::*;
use rpc::zeus_data::RowResult;
use rpc::zeus_data::QueryRequest;
use rpc::zeus_data::PlanNode;
use rpc::zeus_data::PlanNodeType;
use scheduler::Task;
use storage::column::ColumnValueIter;
use server::ServerContext;
use server::data_service::Rows;
use exec::table_scan_node::TableScanNode;

pub struct ColumnWithInfo {
  pub name: String,
  pub id: Option<i32>,
  pub column: CowPtr<Column>,
}

impl ColumnWithInfo {
  pub fn from(column: Box<Column>) -> ColumnWithInfo {
    ColumnWithInfo {
      name: "".to_string(),
      id: None,
      column: CowPtr::Owned(column),
    }
  }
}

pub struct Block {
  pub columns: Vec<ColumnWithInfo>,
  pub eof: bool,
}

impl Block {
  pub fn from(columns: Vec<ColumnWithInfo>) -> Block {
    Block {
      columns,
      eof: true,
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

impl DAGExecutor {
  pub fn task(
    query_request: QueryRequest,
    sender: Sender<Result<Rows>>,
    server_context: ServerContext,
  ) -> Task
  {
    let task_body = box move || {
      let root_result =
        DAGExecutor::build_plan(query_request.get_plan().get_root(), &server_context);

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

  fn build_plan(
    root: &PlanNode,
    server_context: &ServerContext,
  ) -> Result<Box<ExecNode>>
  {
    if root.get_children().len() == 0 {
      DAGExecutor::build_plan_node(root, server_context)
    } else {
      //TODO: Remove this.
      unreachable!()
    }
  }

  fn build_plan_node(
    leaf_node: &PlanNode,
    server_context: &ServerContext,
  ) -> Result<Box<ExecNode>>
  {
    Ok(match leaf_node.get_plan_node_type() {
      PlanNodeType::SCAN_NODE => TableScanNode::new(leaf_node.get_scan_node(), server_context)?,
    })
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
        block.columns.iter().map(|c| c.column.into_iter()).collect();

      loop {
        let mut incomplete = 0usize;
        let mut row = RowResult::new();
        for column_iter in &mut column_iterators {
          match column_iter.next() {
            Some(t) => row.columns.push(t),
            None => {
              incomplete += 1;
              break;
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
  use rpc::zeus_data::ColumnValue;
  use rpc::zeus_meta::ColumnType;
  use storage::column::column_vector::ColumnVector;
  use util::errors::*;

  struct MemoryBlocks {
    blocks: Vec<Block>,
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
    let column1 = ColumnVector::create(ColumnType::BOOL, vec![true, false]).ok().unwrap();
    let column2 = ColumnVector::create(ColumnType::INT64, vec![12i64, 14i64]).ok().unwrap();
    let block1 = vec![ColumnWithInfo::from(box column1), ColumnWithInfo::from(box column2)];
    let block1 = Block::from(block1);

    let column3 = ColumnVector::create(ColumnType::BOOL, vec![false, true]).ok().unwrap();
    let column4 = ColumnVector::create(ColumnType::INT64, vec![100000i64, 54321i64]).ok().unwrap();
    let block2 = vec![ColumnWithInfo::from(box column3), ColumnWithInfo::from(box column4)];
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
