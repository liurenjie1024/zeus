use std::vec::Vec;

use rpc::zeus_meta::FieldType;
use db::column::Column;
use util::cow_ptr::ToBoxedOwned;
use std::any::Any;

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

impl ToBoxedOwned for ColumnString {
    fn to_boxed_owned(&self) -> Box<Any> {
        Box::new(ColumnString {
            offsets: self.offsets.clone(),
            chars: self.chars.clone()
        })
    }
}

