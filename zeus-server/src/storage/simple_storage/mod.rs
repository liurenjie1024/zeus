mod simple_column_factory;
mod simple_file_segment;

use std::collections::LinkedList;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


use server::config::StorageConfig;
use util::error::Result;
use storage::Storage;
use storage::ScanContext;
use storage::BlockInputStream;
use storage::block_input_stream::CombinedBlockInputStream;
use self::simple_file_segment::SimpleFileSegment;

#[allow(dead_code)]
const TABLE_PLAYLIST_FILE: &'static str = "table.pl";

struct SimpleTable {
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
    let mut playlist_path = PathBuf::from(config.path.clone());
    playlist_path.push(table_id.to_string());
    playlist_path.push(TABLE_PLAYLIST_FILE);

    let playlist_file = File::open(playlist_path)?;
    let mut playlist_file = BufReader::new(playlist_file);

    let mut segments = LinkedList::new();

    loop {
      let mut line = "".to_string();
      let result = playlist_file.read_line(&mut line)?;

      if result == 0 {
        break;
      }

      let mut seg_path = PathBuf::from(config.path.clone());
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
}
