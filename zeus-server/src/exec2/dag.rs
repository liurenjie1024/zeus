use super::exec::ExecNode;
use super::exec::ExecContext;

use super::table_scan_exec_node::TableScanExecNode;

use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use rpc::zeus_data::QueryRequest;
use rpc::zeus_data::RowResult;
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