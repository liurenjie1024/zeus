use std::path::Path;
use std::vec::Vec;
use std::fs::File;

use exec2::exec::ExecNode;
use exec2::exec::ExecContext;
use exec2::block::Block;
use super::blizard_segment::BlizardSegment;
use super::reader::ParquetReader;

use exec::expression::Expr;

use parquet::file::reader::SerializedFileReader as ParquetFileReader;


use util::errors::*;

type BlizardReader = ParquetReader<ParquetFileReader>;

struct BlizardScanNode {
  filter: Option<Expr>,
  segments: Vec<BlizardSegment>,
  cur_segment_idx: usize,
  cur_block_idx: usize,
  projections: Vec<String>,
  column_indices: Vec<usize>,
  cur_reader: Option<BlizardReader>
}



impl ExecNode for BlizardScanNode {
  fn open(&mut self, ctx: ExecContext) -> Result<()> {
    self.cur_segment_idx = 0;
    self.cur_block_idx = 0;
    self.create_reader()?;
    self.init_column_indices()?;
    Ok(())
  }

  fn next(&mut self) -> Result<Block> {
    let reader = self.check_reader()?;

    // Loop until we find the first block
    loop {
      debug!("Reading parquet file: {:?}, row group: {:?}.",
        self.segments[self.cur_segment_idx].get_path(),
        self.cur_block_idx);
      let row_group_reader = reader.get_row_group(self.cur_block_idx)?;
      let row_group_metadata = row_group_reader.metadata();
      let row_num = row_group_metadata.num_rows();
      debug!("There are {:?} in parquet file [{:?}], row group [{:?}]",
        self.segments[self.cur_segment_idx].get_path(),
        self.cur_block_idx,
        row_num);

      let need_scan = self.filter
        .map(|e| e.filter(row_group_metadata.clone()))
        .unwrap_or(true);

      if need_scan {
        let block = reader.read_row_group(self.cur_block_idx, self.column_indices.as_slice())?;
        let is_eof = self.is_eof()?;


      }
    }
  }

  fn close(&mut self) -> Result<()> {
    unimplemented!()
  }
}

impl BlizardScanNode {
  fn create_reader(&mut self) -> Result<()> {
    let cur_path = self.segments[self.cur_segment_idx].get_path();
    self.cur_reader = Some(BlizardReader::open(cur_path)?);
    Ok(())
  }

  fn check_reader(&self) -> Result<&BlizardReader> {
    ensure!(self.cur_reader.is_some(), "Reader should not be empty");
    Ok(self.cur_reader.as_ref().unwrap())
  }

  fn init_column_indices(&mut self) -> Result<()> {
    let schema = self.check_reader().map(|r| r.read_schema())??;

    self.column_indices = self.projections
      .iter()
      .try_fold(Vec::new(), |mut indices, projection| -> Result<Vec<usize>> {
        let column_idx = schema.columns()
          .iter()
          .enumerate()
          .find(|f| f.1.name() == projection)
          .map(|t| t.0)
          .ok_or_else(|| ErrorKind::ColumnNameNotFound(projection.clone()))?;

        indices.push(column_idx);
        Ok(indices)
      })?;

    Ok(())
  }

  fn is_eof(&self) -> Result<bool> {
    self.check_reader()
      .and_then(|r| r.num_of_row_groups())
      .map(|num_row_groups| {
        ((self.cur_segment_idx + 1) == self.segments.len()) &&
          ((self.cur_block_idx+1) == num_row_groups)
      })
  }

  fn move_to_next_block(&mut self) -> Result<()> {
    let is_last_block = self.check_reader()
      .and_then(|r| r.num_of_row_groups())
      .map(|n| (self.cur_block_idx+1) == n)?;

    if is_last_block
  }
}

