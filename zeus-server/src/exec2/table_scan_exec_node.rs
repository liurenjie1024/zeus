use std::vec::Vec;

use super::exec::ExecNode;

use server::ServerContext;
use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;
use storage::ScanContext;

use util::errors::*;

pub(super) struct TableScanExecNode {
}

impl TableScanExecNode {
  pub fn new(plan_node: &PlanNode, server_context: &ServerContext,
    children: Vec<Box<ExecNode>>) -> Result<Box<ExecNode>> {
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::SCAN_NODE,
      "Plan node type must be scan");
    ensure!(children.len() == 0, "Number of table scan node's children should be 0.");

    let scan_node = plan_node.get_scan_node();
    match server_context.get_storage_manager().get_table(scan_node.table_id) {
      Some(table) => {
        let scan_context = ScanContext {
          scan_node,
          catalog_manager: server_context.get_catalog_manager(),
        };

        table.scan2(&scan_context)
      },
      None => {
        error!("Table id {} not found.", scan_node.table_id);
        Err(ErrorKind::TableIdNotFound(scan_node.table_id).into())
      }
    }
  }
}