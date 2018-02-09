use std::vec::Vec;
use std::borrow::ToOwned;

use super::Column;
use rpc::zeus_meta::FieldType;

pub struct ColumnVector<T> {
    field_type: FieldType,
    data: Vec<T>
}

impl<T: Clone> Column for ColumnVector<T> {
    fn size(&self) -> usize {
        self.data.len()
    }

    fn field_type(&self) -> FieldType {
        self.field_type
    }

}

impl<T: Clone> ToOwned for ColumnVector<T> {
    type Owned = Self;

    fn to_owned(&self) -> Self::Owned {
        ColumnVector {
            field_type: self.field_type,
            data: self.data.clone()
        }
    }
}

