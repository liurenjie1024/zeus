mod simple_column_factory;
mod simple_file_segment;

use std::collections::LinkedList;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


use server::config::StorageConfig;
use util::errors::*;
use storage::Storage;
use storage::ScanContext;
use storage::BlockInputStream;
use storage::block_input_stream::CombinedBlockInputStream;
use self::simple_file_segment::SimpleFileSegment;

#[allow(dead_code)]
const TABLE_PLAYLIST_FILE: &'static str = "table.pl";

pub struct SimpleTable {
  table_id: i32,
  file_segments: LinkedList<SimpleFileSegment>,
}

unsafe impl Send for SimpleTable {}
unsafe impl Sync for SimpleTable {}

#[allow(dead_code)]
impl SimpleTable {
  #[allow(dead_code)]
  pub fn new(
    config: &StorageConfig,
    table_id: i32,
  ) -> Result<SimpleTable>
  {
    let mut playlist_path = PathBuf::from(&config.root_path);
    playlist_path.push(table_id.to_string());
    playlist_path.push(TABLE_PLAYLIST_FILE);

    let err_msg = format!("Failed to open playlist file: {:?}", playlist_path);
    let playlist_file = File::open(&playlist_path)
      .chain_err(move || err_msg)?;
    let mut playlist_file = BufReader::new(playlist_file);

    let mut segments = LinkedList::new();

    loop {
      let mut line = "".to_string();

      let err_msg = format!("Failed to read line from: {:?}", playlist_path);
      let result = playlist_file.read_line(&mut line)
        .chain_err(move || err_msg) ?;
      debug!("read one line from {:?}: {:?}", playlist_path, line);

      if result == 0 {
        break;
      }

      line = line.trim().to_string();

      let mut seg_path = PathBuf::from(&config.root_path);
      seg_path.push(table_id.to_string());
      seg_path.push(line);

      let seg = SimpleFileSegment {
        table_id,
        path: seg_path.to_str().unwrap().to_string(),
      };

      seg.validate()?;

      segments.push_back(seg);
    }

    Ok(SimpleTable {
      table_id,
      file_segments: segments,
    })
  }
}

impl Storage for SimpleTable {
  fn get_id(&self) -> i32 {
    self.table_id
  }

  fn scan(
    &self,
    scan_context: &ScanContext,
  ) -> Result<Box<BlockInputStream>>
  {
    assert_eq!(self.table_id, scan_context.scan_node.table_id);

    let streams: Result<Vec<Box<BlockInputStream>>> = self
      .file_segments
      .iter()
      .map(|s| s.scan(&scan_context))
      .try_fold(Vec::new(), |mut cs: Vec<Box<BlockInputStream>>, s| {
        cs.push(s?);
        Ok(cs)
      });

    Ok(Box::new(CombinedBlockInputStream::new(streams?)))
  }

  fn get_row_count(&self) -> Result<i64> {
    Ok(9)
  }
}
