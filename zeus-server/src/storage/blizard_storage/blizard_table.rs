use std::collections::LinkedList;
use std::path::PathBuf;
use std::fs;

use super::super::storage::Storage;
use super::super::storage::ScanContext;
use super::super::block_input_stream::BlockInputStream;
use super::blizard_segment::BlizardSegment;
use storage::block_input_stream::CombinedBlockInputStream;
use server::config::ZeusConfig;
use util::errors::*;

const DATA_FILE_SUFFIX: &'static str = "parquet";

pub struct BlizardTable {
  table_id: i32,
  segments: LinkedList<BlizardSegment>
}

unsafe impl Sync for BlizardTable {}
unsafe impl Send for BlizardTable {}

impl BlizardTable {
  pub fn open(config: &ZeusConfig, table_id: i32) -> Result<BlizardTable> {
    let mut table_root = PathBuf::from(&config.storage.root_path);
    table_root.push(table_id.to_string());

    let segments = fs::read_dir(&table_root)?
      .filter_map(|e| e.ok())
      .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
      .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some(DATA_FILE_SUFFIX))
      .try_fold(LinkedList::new(), |mut segments, entry| -> Result<LinkedList<BlizardSegment>> {
        let segment = BlizardSegment::open(&entry.path())?;
        segments.push_back(segment);
        Ok(segments)
      })?;

    Ok(BlizardTable {
      table_id,
      segments
    })
  }
}

impl Storage for BlizardTable {
  fn get_id(&self) -> i32 {
    self.table_id
  }

  fn scan(&self, scan_context: &ScanContext) -> Result<Box<BlockInputStream>> {
    assert_eq!(self.table_id, scan_context.scan_node.table_id);

    let streams: Result<Vec<Box<BlockInputStream>>> = self.segments
      .iter()
      .map(|s| s.scan(&scan_context))
      .try_fold(Vec::new(), |mut cs: Vec<Box<BlockInputStream>>, s| {
        cs.push(s?);
        Ok(cs)
      });

    Ok(Box::new(CombinedBlockInputStream::new(streams?)))
  }
}