use arrow::record_batch::RecordBatch;

pub struct Block {
  records: RecordBatch,
  eof: bool
}

