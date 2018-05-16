pub mod column_data;

use std::iter::Iterator;
use std::slice::Iter;
use std::convert::From;

use self::column_data::VecColumnData;
use self::column_data::Datum;
use rpc::zeus_meta::ColumnType;
use rpc::zeus_meta::ColumnValue;
use util::errors::*;


pub struct Column {
  field_type: ColumnType,
  data: VecColumnData
}

pub struct ColumnValueIter<'a> {
  data: Iter<'a, Datum>
}

impl Column {
  pub fn new(field_type: ColumnType, data: VecColumnData) -> Column {
    Column {
      field_type,
      data
    }
  }

  pub fn size(&self) -> usize {
    self.data.len()
  }

  pub fn field_type(&self) -> ColumnType {
    self.field_type
  }

  pub fn column_value_iter(&self) -> ColumnValueIter {
    ColumnValueIter {
      data: self.data.datums.iter()
    }
  }

  pub fn iter(&self) -> Iter<Datum> {
    self.data.datums.iter()
  }

  pub fn take(&self, num: usize) -> Column {
    let data = self.data.datums.iter()
      .take(num)
      .map(|x| x.clone())
      .collect::<Vec<Datum>>();

    Column {
      field_type: self.field_type,
      data: VecColumnData::from(data)
    }
  }

  pub fn get(&self, idx: usize) -> Option<Datum> {
    self.data.datums.get(idx).map(|d| d.clone())
  }

  pub fn filter(&self, masks: &Column) -> Result<Column> {
    ensure!(masks.field_type() == ColumnType::BOOL, "filed type must be bool!");
    ensure!(masks.size() == self.size(), "size must be equal!");

    let datums: Vec<Datum> = self.data.datums.iter()
      .zip(masks.iter())
      .filter(|t| {
        match t.1 {
          &Datum::Bool(v) => v,
          _ => false
        }
      })
      .map(|t| t.0.clone())
      .collect();

    Ok(Column {
      field_type: self.field_type,
      data: VecColumnData::from(datums)
    })
  }
}

impl<'a> Iterator for ColumnValueIter<'a> {
  type Item = ColumnValue;
  fn next(&mut self) -> Option<ColumnValue> {
    self.data.next()
      .map(|x| x.into())
  }
}

pub trait ColumnFactory: Send + 'static {
  fn create_column(
    &mut self,
    raw_data: &[u8],
  ) -> Result<Column>;
}
