use std::boxed::Box;
use std::vec::Vec;
use std::borrow::Cow;
use std::clone::Clone;

use db::column::Column;
use util::cow_ptr::CowPtr;
use util::error::Result;


pub struct ColumnWithName {
    name: String,
    column: CowPtr<Column>
}

pub struct Block {
    columns: Vec<ColumnWithName>
}


pub struct ExecContext {
}

trait ExecNode {
    fn open(&mut self, context: &mut ExecContext) -> Result<()>;
    fn next(&mut self) -> Block;
    fn close(&mut self) -> Result<()>;
}

pub struct DAGExecutor {
    root: Box<ExecNode>
}


