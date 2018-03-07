use std::vec::Vec;
use std::borrow::ToOwned;
use std::boxed::Box;
use std::any::Any;

use super::Column;
use rpc::zeus_meta::FieldType;
use util::cow_ptr::ToBoxedOwned;
use util::error::Result;

pub struct ColumnVector<T> {
    field_type: FieldType,
    data: Vec<T>
}

impl<T: Clone + 'static> Column for ColumnVector<T> {
    fn size(&self) -> usize {
        self.data.len()
    }

    fn field_type(&self) -> FieldType {
        self.field_type
    }
}

impl<T: Clone + 'static> ToBoxedOwned for ColumnVector<T> {
    fn to_boxed_owned(&self) -> Box<Any> {
        Box::new(ColumnVector {
            field_type: self.field_type,
            data: self.data.clone()
        })
    }
}

impl<T> ColumnVector<T> {
    pub fn create(field_type: FieldType, data: Vec<T>) -> Result<ColumnVector<T>> {
        Ok(ColumnVector {
            field_type,
            data
        })
    }
}

