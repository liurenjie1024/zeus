use arrow::record_batch::RecordBatch;

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
}
