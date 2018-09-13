use std::iter::Iterator;
use std::sync::Arc;
use std::vec::Vec;

use arrow::record_batch::RecordBatch;
use arrow::datatypes::Field;
use arrow::datatypes::Schema;
use arrow::array::Array;

use super::column::to_column;
use rpc::zeus_data::RowResult;
use util::errors::*;


pub struct Block {
  records: Option<RecordBatch> ,
  eof: bool
}

impl Block {
  pub fn new(records: Option<RecordBatch>, eof: bool) -> Block {
    Block {
      records,
      eof
    }
  }

  pub fn empty() -> Block {
    Block {
      records: None,
      eof: true
    }
  }

  pub fn is_empty(&self) -> bool {
    self.records.is_none()
  }

  pub fn eof(&self) -> bool {
    self.eof
  }

  pub fn set_eof(&mut self, eof: bool) {
    self.eof = eof;
  }

  pub fn iter(&self) -> impl Iterator<Item=Result<RowResult>> + '_ {
    BlockRowIterator {
      row: 0,
      record_batch: self.records.as_ref()
    }
  }

  pub fn get_column_by_name(&self, name: &str) -> Result<Self> {
    match &self.records {
      Some(r) => {
        r.schema().column_with_name(name)
          .map(|t| {
            let idx = t.0;
            let record_batch = RecordBatchBuilder::new()
              .add_field(r.schema().column(idx).clone())
              .add_column(r.column(idx).clone())
              .build()?;
            Ok(Block::new(Some(record_batch), false))
          }).unwrap_or_else(|| Err(format!("Column not found: {}", name).into()))
      },
      None => {
        bail!("Block is empty!")
      }
    }
  }

  pub fn get_column(&self, idx: usize) -> Result<Arc<Array>> {
    self.records
      .map(|r| r.column(idx).clone())
      .ok_or_else(|| format!("Index {} for column does not exist.", idx))
  }
}

struct BlockRowIterator<'a> {
  row: i64,
  record_batch: Option<&'a RecordBatch>
}

impl<'a> Iterator for BlockRowIterator<'a> {
  type Item = Result<RowResult> ;

  fn next(&mut self) -> Option<Result<RowResult>> {
    match self.record_batch {
      Some(r) if self.row < r.num_rows() => {
        let ret = Some((0..r.num_columns())
          .map(|c| to_column(&**r.column(c)))
          .try_fold(RowResult::new(), |mut rr, c| -> Result<RowResult> {
            rr.mut_columns()
              .push(c?.get_column_value(self.row as usize)?);
            Ok(rr)
          }));
        self.row += 1;
        ret
      },
      _ => None
    }
  }
}


pub struct RecordBatchBuilder {
  fields: Vec<Field>,
  arrays: Vec<Arc<Array>>
}

impl RecordBatchBuilder {
  pub fn new() -> Self {
    RecordBatchBuilder {
      fields: Vec::new(),
      arrays: Vec::new()
    }
  }

  pub fn add_field(mut self, field: Field) -> Self {
    self.fields.push(field);
    self
  }

  pub fn add_column(mut self, array: Arc<Array>) -> Self {
    self.arrays.push(array);
    self
  }

  pub fn build(self) -> Result<RecordBatch> {
    self.sanity_check()
      .map(|s| {
        let schema = Arc::new(Schema::new(s.fields));
        RecordBatch::new(schema, s.arrays)
      })
  }

  fn sanity_check(self) -> Result<Self> {
    ensure!(self.fields.len() == self.arrays.len(),
    "Number of fileds {} does not match with number of columns {}", self.fields.len(),
    self.arrays.len());
    ensure!(self.fields.len() > 0, "Record batch can't be empty!");
    ensure!(self.arrays[0].len() > 0, "Row num can't be zero");

    let row_num = self.arrays[0].len();

    for arr in &self.arrays {
      ensure!(arr.len() == row_num, "Column length {} does not match with row num {}", arr.len(),
        row_num)
    }

    Ok(self)
  }
}


