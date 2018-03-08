pub mod table_scan_node;

use std::boxed::Box;
use std::vec::Vec;
use std::borrow::Cow;
use std::clone::Clone;

use futures::sync::oneshot::Sender;

use storage::column::Column;
use util::cow_ptr::CowPtr;
use util::error::Result;
use rpc::zeus_data::QueryRequest;
use rpc::zeus_data::QueryResult;
use rpc::zeus_data::PlanNode;
use rpc::zeus_data::PlanNodeType;
use server::ServerContext;
use scheduler::Runnable;
use server::data_service::Rows;

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

trait ExecNode: Send + 'static {
    fn open(&mut self, context: &mut ExecContext) -> Result<()>;
    fn next(&mut self) -> Result<Block>;
    fn close(&mut self) -> Result<()>;
}


pub struct DAGExecutor {
    root: Box<ExecNode>,
    sender: Sender<QueryRequest>
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

impl Runnable for DAGExecutor {
    fn run(&mut self) {
        unimplemented!()
    }
}






