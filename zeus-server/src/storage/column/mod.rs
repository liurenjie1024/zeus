pub mod vec_column_data;
pub mod const_column_data;

use std::iter::Iterator;
use std::convert::From;
use std::boxed::Box;

use self::vec_column_data::VecColumnData;
use self::vec_column_data::Datum;
use self::const_column_data::ConstColumnData;
use rpc::zeus_meta::ColumnType;
use rpc::zeus_meta::ColumnValue;
use util::errors::*;


pub struct Column {
  field_type: ColumnType,
  column_data: Box<ColumnData>
}

pub struct ColumnIter<'a> {
  column_data: &'a ColumnData,
  idx: usize
}

impl<'a> Iterator for ColumnIter<'a> {
  type Item = Datum;

  fn next(&mut self) -> Option<Datum> {
    let ret = self.column_data.get(self.idx);
    self.idx += 1;
    ret
  }
}

trait ColumnData: Send {
  fn len(&self) -> usize;
  fn get(&self, idx: usize) -> Option<Datum>;
  fn iter(&self) -> ColumnIter;
}


pub struct ColumnValueIter<'a> {
  data: ColumnIter<'a>
}

impl Column {
  pub fn new_vec(field_type: ColumnType, data: VecColumnData) -> Column {
    Column {
      field_type,
      column_data: box data
    }
  }

  pub fn new_const(field_type: ColumnType, datum: Datum, size: usize) -> Column {
    Column {
      field_type,
      column_data: box ConstColumnData::new(size, datum)
    }
  }

  pub fn size(&self) -> usize {
    self.column_data.len()
  }

  pub fn field_type(&self) -> ColumnType {
    self.field_type
  }

  pub fn column_value_iter(&self) -> ColumnValueIter {
    ColumnValueIter {
      data: self.column_data.iter()
    }
  }

  pub fn iter(&self) -> ColumnIter {
    self.column_data.iter()
  }

  pub fn take(&self, num: usize) -> Column {
    let data = self.iter()
      .take(num)
      .map(|x| x.clone())
      .collect::<Vec<Datum>>();

    Column {
      field_type: self.field_type,
      column_data: box VecColumnData::from(data)
    }
  }

  pub fn get(&self, idx: usize) -> Option<Datum> {
    self.column_data.get(idx).map(|d| d.clone())
  }

  pub fn filter(&self, masks: &Column) -> Result<Column> {
    ensure!(masks.field_type() == ColumnType::BOOL, "filed type must be bool!");
    ensure!(masks.size() == self.size(), "size must be equal!");

    let datums: Vec<Datum> = self.iter()
      .zip(masks.iter())
      .filter(|t| {
        match t.1 {
          Datum::Bool(v) => v,
          _ => false
        }
      })
      .map(|t| t.0.clone())
      .collect();

    Ok(Column {
      field_type: self.field_type,
      column_data: box VecColumnData::from(datums)
    })
  }
}

impl<'a> Iterator for ColumnValueIter<'a> {
  type Item = ColumnValue;
  fn next(&mut self) -> Option<ColumnValue> {
    self.data.next()
      .map(|x| (&x).into())
  }
}

pub trait ColumnFactory: Send + 'static {
  fn create_column(
    &mut self,
    raw_data: &[u8],
  ) -> Result<Column>;
}
