use std::vec::Vec;

use rpc::zeus_meta::FieldType;

use db::column::Column;

pub struct ColumnString {
    offsets: Vec<usize>,
    chars: Vec<u8>
}

impl Column for ColumnString {
    fn size(&self) -> usize {
        self.offsets.len()
    }

    fn field_type(&self) -> FieldType {
        FieldType::STRING
    }
}

