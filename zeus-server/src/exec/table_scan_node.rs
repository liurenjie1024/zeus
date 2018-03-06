use std::vec::Vec;
use std::collections::HashMap;
use std::iter::Iterator;

use util::cow_ptr::CowPtr;
use util::error::Result;
use db::column::Column;
use super::ExecNode;
use super::ExecContext;
use super::Block;
use db::BlockInputStream;
use rpc::zeus_data::ScanNode;
use server::ServerContext;

pub struct TableScanNode {
    db_id: i32,
    table_id: i32,
    input_stream: Box<BlockInputStream>
}

impl TableScanNode {
    pub fn new(scan_node: &ScanNode, server_context: &ServerContext)
        -> Result<Box<ExecNode>> {

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