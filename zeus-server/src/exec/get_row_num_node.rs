use std::vec::Vec;

use server::ServerContext;

use exec::ExecNode;
use exec::ExecContext;
use exec::Block;

use storage::column::ColumnBuilder;
use storage::column::vec_column_data::Datum;
use storage::ErrorKind as DBErrorKind;


use rpc::zeus_meta::ColumnType;
use rpc::zeus_plan::PlanNode;
use rpc::zeus_plan::PlanNodeType;

use util::errors::*;

pub struct GetRowNumExecNode {
  alias: String,
  row_num: i64,
  executed: bool
}

impl ExecNode for GetRowNumExecNode {
  fn open(&mut self, _context: &mut ExecContext) -> Result<()> {
    self.executed = false;
    Ok(())
  }

  fn next(&mut self) -> Result<Block> {
    if self.executed {
      return Err(ErrorKind::EOF.into());
    }

    self.executed = true;
    let column = ColumnBuilder::new_const(ColumnType::INT64,
      Datum::Int64(self.row_num), 1)
      .set_name(self.alias.clone())
      .build();

    Ok(Block::new(vec![column], true))
  }


  fn close(&mut self) -> Result<()> {
    Ok(())
  }
}

impl GetRowNumExecNode {
  pub fn new(plan_node: &PlanNode, server_context: &ServerContext,
    children: Vec<Box<ExecNode>>) -> Result<Box<dyn ExecNode>> {
    ensure!(children.len() == 0, "Children number should be zero for GetRowNumNode");
    ensure!(plan_node.get_plan_node_type() == PlanNodeType::GET_ROW_NUM_NODE,
    "Plan node type must be GET_ROW_NUM_NODE");

    let node = plan_node.get_get_row_num_node();
    let table_id = node.get_table_id();

    let row_num = server_context.get_storage_manager()
      .get_table(table_id)
      .map(|t| t.get_row_count())
      .unwrap_or(Err(ErrorKind::DB(DBErrorKind::TableNotFound).into()))?;

    Ok(box GetRowNumExecNode {
      alias: node.get_alias().to_string(),
      row_num,
      executed: false
    })
  }
}