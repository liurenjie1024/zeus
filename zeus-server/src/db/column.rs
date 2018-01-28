use std::vec::Vec;
use db::schema::FieldInfo;

pub trait Column {
    fn size(&self) -> usize;
    fn field_info(&self) -> &FieldInfo;
}

struct NumericColumn<T: Ord> {
    field_info: FieldInfo,
    values: Vec<T>,
    range_index: RangeIndex<T>
}

struct RangeIndex<T: Ord> {
    min: T,
    max: T
}

impl<T: Ord> Column for NumericColumn<T> {
    fn size(&self) -> usize {
        self.values.len()
    }

    fn field_info(&self) -> &FieldInfo {
        &self.field_info
    }
}


