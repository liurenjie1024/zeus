use std::vec::Vec;

use super::Column;
use rpc::zeus_meta::FieldType;

pub struct ColumnVector<T> {
    field_type: FieldType,
    data: Vec<T>
}

impl<T> Column for ColumnVector<T> {
    fn size(&self) -> usize {
        self.data.len()
    }

    fn field_type(&self) -> FieldType {
        self.field_type
    }
}
