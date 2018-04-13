use std::collections::LinkedList;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use super::super::storage::Storage;
use super::super::storage::ScanContext;
use super::super::block_input_stream::BlockInputStream;
use super::blizard_segment::BlizardSegment;
use server::config::ZeusConfig;
use util::errors::*;

const TABLE_PLAYLIST_FILE: &'static str = "table.pl";

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


    let mut playlist_path = table_root.clone();
    playlist_path.push(TABLE_PLAYLIST_FILE);

    let err_msg = format!("Failed to open playlist file: {:?}", playlist_path);
    let playlist_file = File::open(&playlist_path)
      .chain_err(move || err_msg)?;
    let mut playlist_file = BufReader::new(playlist_file);

    let mut segments = LinkedList::new();

    for line in playlist_file.lines() {
      let name = line.chain_err(|| {
        let err_msg = "Failed to read line";
        error!("{}", err_msg);
        err_msg
      })?;

      name.trim();

      let err_msg = format!("Failed to open segment \"{:?}\" in table root \"{:?}\"", name, table_root);
      let segment = BlizardSegment::open(&table_root, &name)
        .chain_err(move || err_msg)?;

      segments.push_back(segment);
    }

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

  fn scan(&self, _scan_context: &ScanContext) -> Result<Box<BlockInputStream>> {
    unimplemented!()
  }
}