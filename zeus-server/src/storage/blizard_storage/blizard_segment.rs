use std::path::Path;
use std::path::PathBuf;
use std::borrow::ToOwned;
use std::fs::File;
use std::io::Read;
use std::io::SeekFrom;
use std::sync::Arc;
use std::io::Error as StdIoError;
use std::io::ErrorKind as StdIoErrorKind;
use std::collections::HashMap;

use protobuf::parse_from_reader;

use exec::ExecPhase;
use exec::Block;
use storage::BlockInputStream;
use rpc::zeus_meta::ColumnType;
use rpc::zeus_blizard_format::SegmentIndex;
use rpc::zeus_blizard_format::BlockNode;
use util::errors::*;

pub(super) struct BlizardSegment {
  index: Arc<SegmentIndex>,
  data_path: Box<Path>
}

impl BlizardSegment {
  pub fn open<P: AsRef<Path>>(dir: &P, name: &str) -> Result<BlizardSegment> {
    let mut path = PathBuf::new();
    path.push(dir.to_owned());
    path.set_file_name(name);

    // Read index
    path.set_extension("idx");
    let err_msg = format!("Failed to parse open index file: \"{:?}\"", path);
    let mut file = File::open(&path)
      .chain_err(move || {
        error!(err_msg);
        err_msg
      });
    let index = parse_from_reader::<SegmentIndex>(&mut file)?;
    debug!("Segment index read:{:?}", index);

    // Check file data eixsts
    path.set_extension("data");
    if !path.exists() {
      let err_msg = format!("Data file \"{:?}\" not found.", path);
      error!(err_msg);
      Err(StdIoError::new(StdIoErrorKind::NotFound, err_msg))
    }

    Ok(BlizardSegment {
      index,
      data_path: path.into_boxed_path()
    })
  }
}

struct FileSegmentBlockInputStream {
  phase: ExecPhase,
  next_block_idx: usize,
  path: Box<Path> ,
  read: Box<Read>,
  index: Arc<SegmentIndex>,
  column_types: HashMap<i32, ColumnType>,
  column_names: HashMap<i32, String>,
  column_factories: HashMap<i32, Box<ColumnFactory>>,
}

impl SimpleFileSegment {
  #[allow(dead_code)]
  pub fn validate(&self) -> Result<()> {
    let path = Path::new(&self.path);

    if path.exists() && path.is_file() {
      Ok(())
    } else {
      let msg = format!("{} not found.", self.path);
      Err(StdIoError::new(StdIoErrorKind::NotFound, msg))?
    }
  }

  pub fn scan(
    &self,
    context: &ScanContext,
  ) -> Result<Box<BlockInputStream>>
  {
    let blocks = BlockHandles::new();

    let scan_node = context.scan_node;

    let mut column_types: HashMap<i32, ColumnType> = HashMap::new();
    let mut column_names: HashMap<i32, String> = HashMap::new();

    let table_schema =
      context.catalog_manager.get_table_schema(scan_node.table_id).unwrap();
    for column_id in &scan_node.columns {
      let column_schema = table_schema.get_column_schema(*column_id).unwrap();
      column_types.insert(*column_id, column_schema.get_type());
      column_names.insert(*column_id, column_schema.get_name());
    }


    Ok(Box::new(FileSegmentBlockInputStream {
      phase: ExecPhase::UnInited,
      next_block_idx: 0,
      path: self.path.clone(),
      file: None,
      blocks,
      column_types,
      column_names,
      column_factories: HashMap::new(),
    }))
  }
}

impl BlockInputStream for FileSegmentBlockInputStream {
  fn open(&mut self) -> Result<()> {
    assert_eq!(ExecPhase::UnInited, self.phase);

    const MAGIC_NUM: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    const VERSION: [u8; 4] = [00, 00, 00, 01];

    self.file = Some(File::open(&self.path)?);
    let file_ref: &mut File = self.file.as_mut().unwrap();
    let mut header = [0 as u8; 8];
    file_ref.read_exact(&mut header)?;

    if &header[0..4] != MAGIC_NUM || &header[4..] != VERSION {
      bail!(ErrorKind::DB(DBErrorKind::InvalidHeader))
    }

    let mut index_len_buf = [0 as u8; 4];
    file_ref.seek(SeekFrom::End(-4))?;
    file_ref.read_exact(&mut index_len_buf)?;
    let index_len = BigEndian::read_i32(&index_len_buf) as usize;
    debug!("Serialized block chain size: {}", index_len);

    let mut block_handles_buf: Vec<u8> = Vec::with_capacity(index_len);
    for _ in 0..index_len {
      block_handles_buf.push(0 as u8);
    }
    file_ref.seek(SeekFrom::End((index_len as i64) * -1-4))?;
    file_ref.read_exact(&mut block_handles_buf)?;
    debug!("Block bytes are: {:?}", block_handles_buf);

    let block_handles = parse_from_bytes::<BlockHandles>(&block_handles_buf)?;
    debug!("Block handles parsed: {:?}", block_handles);

    self.blocks = block_handles;

    for (column_id, column_type) in self.column_types.iter() {
      self.column_factories.insert(
        *column_id,
        create_column_factory(*column_type, self.blocks.get_max_block_column_size() as usize),
      );
    }

    self.phase = ExecPhase::Opened;
    Ok(())
  }

  fn next(&mut self) -> Result<Block> {
    assert!((ExecPhase::Opened == self.phase) || (ExecPhase::Executed == self.phase));
    if self.next_block_idx >= self.blocks.get_handles().len() {
      bail!(ErrorKind::DB(DBErrorKind::EOF))
    }

    let block_handle = self.blocks.get_handles().get(self.next_block_idx).unwrap();

    for x in self.column_names.keys() {
      assert!(block_handle.columns.contains_key(x), "Column {} doesn't exist in {}.", x, self.path);
    }

    let mut sorted_column_ids = self.column_names.keys().cloned().collect::<Vec<i32>>();
    sorted_column_ids.sort_by_key(|id| block_handle.columns[id].start);

    let mut columns: Vec<ColumnWithInfo> = Vec::new();
    for column_id in &sorted_column_ids {
      let column_handle = block_handle.get_columns().get(column_id).unwrap();
      let column_start = column_handle.get_start() as u64;
      let mut column_factory = self.column_factories.get_mut(column_id).unwrap();
      //TODO: Optimize this
      let buf_len = (column_handle.get_end() - column_handle.get_start()) as usize;
      let mut buf = Vec::with_capacity(buf_len);
      unsafe {
        buf.set_len(buf_len);
      }

      let file_ref: &mut File = self.file.as_mut().unwrap();
      file_ref.seek(SeekFrom::Start(column_start))?;
      file_ref.read_exact(&mut buf)?;

      let column = column_factory.create_column(&buf)?;
      columns.push(ColumnWithInfo {
        name: self.column_names.get(column_id).unwrap().clone(),
        id: Some(*column_id),
        column: CowPtr::Owned(column),
      });
    }

    self.next_block_idx += 1;

    Ok(Block {
      columns,
      eof: self.next_block_idx >= self.blocks.get_handles().len(),
    })
  }

  fn close(&mut self) -> Result<()> {
    if self.phase == ExecPhase::Closed {
      return Ok(());
    }

    self.phase = ExecPhase::Closed;
    Ok(())
  }
}