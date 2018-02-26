pub mod table_scan_node;
pub mod scanner;

use std::boxed::Box;
use std::vec::Vec;
use std::borrow::Cow;
use std::clone::Clone;

use db::column::Column;
use util::cow_ptr::CowPtr;
use util::error::Result;
use rpc::zeus_data::QueryPlan;
use rpc::zeus_data::QueryResult;


pub struct ColumnWithInfo {
    name: String,
    id: Option<i32>,
    column: CowPtr<Column>
}

pub struct Block {
    columns: Vec<ColumnWithInfo>,
    eof: bool
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecPhase {
    UnInited,
    Opened,
    Executed,
    Closed
}

pub struct ExecContext {
}

trait ExecNode {
    fn open(&mut self, context: &mut ExecContext) -> Result<()>;
    fn next(&mut self) -> Result<Block>;
    fn close(&mut self) -> Result<()>;
}


pub struct DAGExecutor {
    root: Box<ExecNode>
}

impl DAGExecutor {
    pub fn from(query_plan: &QueryPlan) -> DAGExecutor {
        unimplemented!()
    }

    pub fn execute(self: &mut DAGExecutor) -> Result<QueryResult> {
        unimplemented!()
    }
}




