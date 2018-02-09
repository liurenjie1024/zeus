use std::boxed::Box;
use std::vec::Vec;

use db::column::Column;


pub struct ColumnWithName {
    name: String,
    column: Box<Column>
}

pub struct Block {
    columns: Vec<ColumnWithName>
}



