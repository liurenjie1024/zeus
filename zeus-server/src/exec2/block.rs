use std::vec::Vec;
use std::iter::Iterator;

use arrow::record_batch::RecordBatch;

use super::column::Column;
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

  pub fn iter(&self) -> impl Iterator<Item=Result<RowResult>> + '_ {
    BlockRowIterator {
      row: 0,
      record_batch: self.records.as_ref()
    }
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

