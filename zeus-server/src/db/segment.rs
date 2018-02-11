use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

use rpc::zeus_data::Row;
use util::collections::concurrent_list::ConcurrentList;
use db::column::Column;



pub struct AppendableSegment {
    row_list: ConcurrentList<Row>
}

impl AppendableSegment {
    pub fn new() -> AppendableSegment {
        AppendableSegment {
            row_list: ConcurrentList::new()
        }
    }
    pub fn append(&self, row: Row) -> usize {
        self.row_list.append(row);
        self.row_list.size()
    }

    pub fn size(&self) -> usize {
        self.row_list.size()
    }
}

pub struct ImmutableSegment {
    columns: HashMap<i32, Box<Column>>,
    row_num: usize
}