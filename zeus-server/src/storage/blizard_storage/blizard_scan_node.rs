use std::vec::Vec;
use std::rc::Rc;

use exec2::exec::ExecNode;
use exec2::exec::ExecContext;
use exec2::block::Block;
use exec2::exec::ExecPhase;
use super::blizard_segment::BlizardSegment;
use super::reader::ParquetReader;

use exec::expression::Expr;

use parquet::file::reader::SerializedFileReader as ParquetFileReader;


use util::errors::*;

type BlizardReader = ParquetReader<ParquetFileReader>;

pub(super) struct BlizardScanNode {
  exec_phase: ExecPhase,
  filter: Option<Expr>,
  segments: Vec<BlizardSegment>,
  cur_segment_idx: usize,
  cur_block_idx: usize,
  projections: Vec<String>,
  column_indices: Vec<usize>,
  cur_reader: Option<Rc<BlizardReader>>
}



impl ExecNode for BlizardScanNode {
  fn open(&mut self, _ctx: &ExecContext) -> Result<()> {
    ensure!(self.exec_phase == ExecPhase::UnInited, "Phase must uninited to open!");
    self.cur_segment_idx = 0;
    self.cur_block_idx = 0;
    self.create_reader()?;
    self.init_column_indices()?;
    self.exec_phase = ExecPhase::Opened;
    Ok(())
  }

  fn next(&mut self) -> Result<Block> {
    ensure!(self.exec_phase == ExecPhase::Opened, "Phase must opened to execute!");
    let reader = self.check_reader()?;

    let mut result = Block::empty();
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
        .as_ref()
        .map(|e| e.filter(row_group_metadata.clone()))
        .unwrap_or(true);

      let eof = self.is_eof()?;

      if need_scan {
        let records = reader.read_row_group(self.cur_block_idx, self.column_indices.as_slice())?;

        result = Block::new(Some(records), eof);
        if !eof {
          self.move_to_next_block()?;
        } else {
          self.clear_reader();
        }
        break;
      } else {
        if eof {
          self.clear_reader();
          break;
        } else {
          self.move_to_next_block()?;
        }
      }
    }

    Ok(result)
  }

  fn close(&mut self) -> Result<()> {
    ensure!(self.exec_phase==ExecPhase::Opened, "Phase must opened to close!");
    self.clear_reader();
    Ok(())
  }
}

impl BlizardScanNode {
  pub fn new(filter: Option<Expr>, segments: Vec<BlizardSegment>, projections: Vec<String>)
    -> BlizardScanNode {
    BlizardScanNode {
      exec_phase: ExecPhase::UnInited,
      filter,
      segments,
      cur_segment_idx: 0usize,
      cur_block_idx: 0usize,
      projections,
      column_indices: Vec::new(),
      cur_reader: None
    }
  }

  fn create_reader(&mut self) -> Result<()> {
    let cur_path = self.segments[self.cur_segment_idx].get_path();
    self.cur_reader = Some(Rc::new(BlizardReader::open(cur_path)?));
    Ok(())
  }

  fn check_reader(&self) -> Result<Rc<BlizardReader>> {
    ensure!(self.cur_reader.is_some(), "Reader should not be empty");
    Ok(self.cur_reader.as_ref().map(|r| r.clone()).unwrap())
  }

  fn clear_reader(&mut self) {
    self.cur_reader = None
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

    if is_last_block {
      self.cur_segment_idx += 1;
      self.cur_block_idx = 0;
      self.create_reader()
    } else {
      self.cur_block_idx += 1;
      Ok(())
    }
  }
}

