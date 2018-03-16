use std::vec::Vec;
use std::collections::HashMap;

use util::cow_ptr::CowPtr;
use util::error::Result;
use util::error::Error;
use storage::ErrorKind as DBErrorKind;
use storage::column::Column;
use storage::storage::ScanContext;
use super::ExecNode;
use super::ExecContext;
use super::Block;
use storage::BlockInputStream;
use rpc::zeus_data::ScanNode;
use server::ServerContext;

pub struct TableScanNode {
  table_id: i32,
  input_stream: Box<BlockInputStream>,
}

impl TableScanNode {
  pub fn new(scan_node: &ScanNode, server_context: &ServerContext)
             -> Result<Box<ExecNode>> {
    match server_context.get_storage_manager().get_table(scan_node.table_id) {
      Some(table) => {
        let scan_context = ScanContext {
          scan_node,
          catalog_manager: server_context.get_catalog_manager()
        };

        Ok(box TableScanNode {
          table_id: scan_node.table_id,
          input_stream: table.scan(&scan_context)?
        })
      },

      None => {
        error!("Table id {} not found.", scan_node.table_id);
        Err(Error::DBError(DBErrorKind::TableNotFound))
      }
    }
  }
}

impl ExecNode for TableScanNode {
  fn open(&mut self, context: &mut ExecContext) -> Result<()> {
    debug!("Begin to scan..");
    self.input_stream.open()
  }

  fn next(&mut self) -> Result<Block> {
    debug!("Begin to get block..");
    self.input_stream.next()
  }

  fn close(&mut self) -> Result<()> {
    debug!("Close scan node..");
    self.input_stream.close()
  }
}