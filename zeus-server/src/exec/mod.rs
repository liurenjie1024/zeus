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
use rpc::zeus_data::PlanNode;
use rpc::zeus_data::PlanNodeType;
use rpc::zeus_data::QueryResult;
use server::ServerContext;


pub struct ColumnWithInfo {
    pub name: String,
    pub id: Option<i32>,
    pub column: CowPtr<Column>
}

pub struct Block {
    pub columns: Vec<ColumnWithInfo>,
    pub eof: bool
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
    pub fn from(query_plan: &QueryPlan, server_context: &ServerContext) -> DAGExecutor {
        unimplemented!()
    }

    fn build_plan(query_plan: &QueryPlan, cur: usize, server_context: &ServerContext)
        -> Result<(Box<ExecNode>, usize)> {

    }

    fn build_plan_node(query_plan: &QueryPlan, cur: usize, server_context: &ServerContext)
        -> Result<Box<ExecNode>> {
        assert!(cur < query_plan.get_nodes().len());

        let plan_node: &PlanNode = query_plan.get_nodes().get(cur).unwrap();
        match plan_node.get_plan_node_type() {
            PlanNodeType::SCAN_NODE =>

        }

    }

    pub fn execute(self: &mut DAGExecutor) -> Result<QueryResult> {
        unimplemented!()
    }
}




