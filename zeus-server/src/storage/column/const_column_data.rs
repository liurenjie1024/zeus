use super::vec_column_data::Datum;
use super::ColumnData;
use super::ColumnIter;

pub struct ConstColumnData {
  size: usize,
  data: Datum
}

impl ColumnData for ConstColumnData {
  fn len(&self) -> usize {
    self.size
  }

  fn get(&self, idx: usize) -> Option<Datum> {
    if idx < self.size {
      Some(self.data.clone())
    } else {
      None
    }

  }

  fn iter(&self) -> ColumnIter {
    ColumnIter {
      column_data: self,
      idx: 0 as usize
    }
  }
}

impl ConstColumnData {
  pub fn new(size: usize, data: Datum) -> ConstColumnData {
    ConstColumnData {
      size,
      data
    }
  }
}

