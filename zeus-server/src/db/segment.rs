use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

use db::column::Column;

pub struct MemSegment {
    columns: HashMap<i32, Box<Column>>,
    row_num: usize
}