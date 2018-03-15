pub mod table_scan_node;

use std::boxed::Box;
use std::vec::Vec;
use std::borrow::Cow;
use std::clone::Clone;
use std::convert::Into;

use futures::sync::oneshot::Sender;

use storage::column::Column;
use util::cow_ptr::CowPtr;
use util::error::Result;
use rpc::zeus_data::RowResult;
use rpc::zeus_data::QueryRequest;
use rpc::zeus_data::QueryResult;
use rpc::zeus_data::PlanNode;
use rpc::zeus_data::PlanNodeType;
use scheduler::Task;
use storage::column::ColumnValueIter;
use server::ServerContext;
use server::data_service::Rows;

pub struct ColumnWithInfo {
  pub name: String,
  pub id: Option<i32>,
  pub column: CowPtr<Column>,
}

pub struct Block {
  pub columns: Vec<ColumnWithInfo>,
  pub eof: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecPhase {
  UnInited,
  Opened,
  Executed,
  Closed,
}

pub struct ExecContext {}

trait ExecNode: Send + 'static {
  fn open(&mut self, context: &mut ExecContext) -> Result<()>;
  fn next(&mut self) -> Result<Block>;
  fn close(&mut self) -> Result<()>;
}


pub struct DAGExecutor {
  root: Box<ExecNode>,
  sender: Sender<Result<Rows>>,
}

unsafe impl Send for DAGExecutor {}

impl DAGExecutor {
  pub fn from(query_request: QueryRequest,
              sender: Sender<Result<Rows>>,
              server_context: ServerContext) -> DAGExecutor {
    unimplemented!()
  }

//    fn build_plan(query_plan: &QueryPlan, cur: usize, server_context: &ServerContext)
//        -> Result<(Box<ExecNode>, usize)> {
//        unimplemented!()
//    }
//
//    fn build_plan_node(query_plan: &QueryPlan, cur: usize, server_context: &ServerContext)
//        -> Result<Box<ExecNode>> {
////        assert!(cur < query_plan.get_nodes().len());
//        unimplemented!()
////
////        let plan_node: &PlanNode = query_plan.get_nodes().get(cur).unwrap();
////        match plan_node.get_plan_node_type() {
////            PlanNodeType::SCAN_NODE =>
////
////        }
//    }
}

impl DAGExecutor {
  fn run(mut self) {
    let result = self.query();
    match self.sender.send(result) {
      Ok(_) => info!("succeeded to send query result!"),
      Err(_) => error!("Failed to send query result!")
    }
  }

  fn query(&mut self) -> Result<Rows> {
    let mut context = ExecContext {};

    self.root.open(&mut context)?;

    let mut rows = Rows::new();

    loop {
      let block = self.root.next()?;

      let mut column_iterators: Vec<ColumnValueIter> = block.columns.iter()
        .map(|c| c.column.iter())
        .collect();

      loop {
        let mut incomplete = 0usize;
        let mut row = RowResult::new();
        for column_iter in &mut column_iterators {
          match column_iter.next() {
            Some(t) => row.columns.push(t),
            None => {
              incomplete += 1;
              break;
            }
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

impl Into<Task> for DAGExecutor {
  fn into(self) -> Task {
    let task_body = Box::new(move || { self.run() });
    Task::new(task_body)
  }
}








