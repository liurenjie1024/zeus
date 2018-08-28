use super::exec::ExecNode;
use super::exec::ExecContext;

use super::table_scan_exec_node::TableScanExecNode;

use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use rpc::zeus_data::QueryRequest;
use server::data_service::Rows;
use server::ServerContext;

use scheduler::Task;

use futures::sync::oneshot::Sender;

use util::errors::*;

pub struct DAGExecutor {
  root: Box<ExecNode>,
  sender: Sender<Result<Rows>>,
}


impl PlanNode {
  pub fn to_exec_node2(&self, server_context: &ServerContext) -> Result<Box<ExecNode>> {
    let children = self.get_children()
      .iter()
      .try_fold(Vec::new(), |mut res, plan_node| -> Result<Vec<Box<ExecNode>>> {
        res.push(plan_node.to_exec_node2(server_context)?);
        Ok(res)
      })?;

    match self.get_plan_node_type() {
      PlanNodeType::SCAN_NODE => TableScanExecNode::new(&self, server_context, children),
      plan_node_type => bail!("Unimplemented plan node type: {:?}", plan_node_type)
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
      let root_result = query_request.get_plan().get_root().to_exec_node2(&server_context);

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

      let mut block_rows = block.iter()
        .try_fold(Rows::new(), |mut rows, r| -> Result<Rows> {
          rows.push(r?);
          Ok(rows)
        })?;

      rows.append(&mut block_rows);

      if block.eof() {
        break
      }
    }

    Ok(rows)
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use arrow::datatypes::Field;
  use arrow::datatypes::DataType;
  use arrow::datatypes::Schema;
  use arrow::array::Array;
  use arrow::array::PrimitiveArray;
  use arrow::record_batch::RecordBatch;

  use futures::sync::oneshot::channel;
  use futures::Future;
  use futures::Async;

  use super::super::block::Block;
  use super::super::exec::ExecNode;
  use super::super::exec::ExecContext;
  use super::DAGExecutor;

  use rpc::zeus_data::RowResult;
  use rpc::zeus_meta::ColumnValue;

  use util::errors::*;

  struct MemoryBlocks {
    blocks: Vec<Block>
  }

  impl ExecNode for MemoryBlocks {
    fn open(&mut self, _ctx: &ExecContext) -> Result<()> {
      Ok(())
    }

    fn next(&mut self) -> Result<Block> {
      assert!(!self.blocks.is_empty());

      let mut block = self.blocks.pop().unwrap();
      block.set_eof(self.blocks.is_empty());
      Ok(block)
    }

    fn close(&mut self) -> Result<()> {
      Ok(())
    }
  }

  impl Default for MemoryBlocks {
    fn default() -> Self {
      MemoryBlocks {
        blocks: vec![MemoryBlocks::block1(), MemoryBlocks::block2()]
      }
    }
  }

  impl MemoryBlocks {
    fn block1() -> Block {
      let schema = Arc::new(Schema::new(
        vec![Field::new("salary", DataType::Int32, false),
          Field::new("fee", DataType::Int64, false)]));

      let arrays: Vec<Arc<Array>> = vec![Arc::new(PrimitiveArray::<i32>::from(vec![12i32, 14i32])),
        Arc::new(PrimitiveArray::<i64>::from(vec![10000i64, 20000i64]))];

      let record_batch = RecordBatch::new(schema, arrays);

      Block::new(Some(record_batch), false)
    }

    fn block2() -> Block {
      let schema = Arc::new(Schema::new(
        vec![Field::new("salary", DataType::Int32, false),
          Field::new("fee", DataType::Int64, false)]));

      let arrays: Vec<Arc<Array>> = vec![Arc::new(PrimitiveArray::<i32>::from(vec![17i32, 19i32])),
        Arc::new(PrimitiveArray::<i64>::from(vec![30003i64, 50005i64]))];

      let record_batch = RecordBatch::new(schema, arrays);

      Block::new(Some(record_batch), false)
    }
  }

  #[test]
  fn test_run_dag() {
    let (sender, mut receiver) = channel();
    let dag = DAGExecutor {
      root: box MemoryBlocks::default(),
      sender,
    };

    dag.run();
    let result = receiver.poll().unwrap();

    let expected_rows = vec![
      to_row_result(17i32, 30003i64),
      to_row_result(19i32, 50005i64),
      to_row_result(12i32, 10000i64),
      to_row_result(14i32, 20000i64),
    ];

    match result {
      Async::Ready(t) => assert_eq!(expected_rows, t.ok().unwrap()),
      Async::NotReady => assert!(false, "Result should be ready!"),
    }
  }

  fn to_row_result(
    v1: i32,
    v2: i64,
  ) -> RowResult
  {
    let mut r = RowResult::new();

    let mut c1 = ColumnValue::new();
    c1.set_i32_value(v1);

    let mut c2 = ColumnValue::new();
    c2.set_i64_value(v2);

    r.mut_columns().push(c1);
    r.mut_columns().push(c2);

    r
  }

}