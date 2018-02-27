use std::vec::Vec;
use std::any::Any;

use rpc::zeus_meta::FieldType;
use db::column::Column;
use util::error::Result;
use util::cow_ptr::ToBoxedOwned;

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

impl ColumnString {
    pub fn new(offsets: Vec<usize>, chars: Vec<u8>) -> Result<ColumnString> {
        Ok(ColumnString {
            offsets,
            chars
        })
    }
}
