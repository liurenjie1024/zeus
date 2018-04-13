use std::path::Path;
use std::path::PathBuf;
use std::borrow::ToOwned;
use std::io::Read;
use std::fs::File;
use std::sync::Arc;
use std::io::Error as StdIoError;
use std::io::ErrorKind as StdIoErrorKind;
use std::collections::HashMap;

use protobuf::parse_from_reader;

use exec::ExecPhase;
use exec::Block;
use exec::ColumnWithInfo;
use storage::ScanContext;
use storage::BlockInputStream;
use storage::ErrorKind as DBErrorKind;
use rpc::zeus_meta::ColumnType;
use rpc::zeus_blizard_format::SegmentIndex;
use rpc::zeus_blizard_format::BlockNode;
use rpc::zeus_blizard_format::ColumnNode;
use storage::column::arrow_column::ArrowColumn;
use util::cow_ptr::CowPtr;
use util::errors::*;

const EXT_INDEX: &'static str = "idx";
const EXT_DATA: &'static str = "bin";

pub(super) struct BlizardSegment {
  index: Arc<SegmentIndex>,
  data_path: Box<Path>,
  name: String
}

impl BlizardSegment {
  pub fn open<P: AsRef<Path>>(dir: &P, name: &str) -> Result<BlizardSegment> {
    let mut path = PathBuf::new();
    path.push(dir.to_owned());
    path.set_file_name(name);

    // Read index
    path.set_extension(EXT_INDEX);
    let err_msg = format!("Failed to parse open index file: \"{:?}\"", path);
    let mut file = File::open(&path)
      .chain_err(move || {
        error!("{}", err_msg);
        err_msg
      })?;
    let index = parse_from_reader::<SegmentIndex>(&mut file)?;
    debug!("Segment index read:{:?}", index);

    // Check file data eixsts
    path.set_extension(EXT_DATA);
    if !path.exists() {
      let err_msg = format!("Data file \"{:?}\" not found.", path);
      error!("{}", err_msg);
      return Err(Error(StdIoError::new(StdIoErrorKind::NotFound, err_msg)));
    }

    Ok(BlizardSegment {
      index: Arc::new(index),
      data_path: path.into_boxed_path(),
      name: name.to_string()
    })
  }
}

struct FileSegmentBlockInputStream {
  phase: ExecPhase,
  next_block_idx: usize,
  path: Arc<PathBuf> ,
  reader: Option<File>,
  index: Arc<SegmentIndex>,
  column_types: HashMap<i32, ColumnType>,
  column_names: HashMap<i32, String>,
}

impl BlizardSegment {
  pub fn scan(
    &self,
    context: &ScanContext,
  ) -> Result<Box<BlockInputStream>>
  {
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

    let mut path = self.data_path.into_path_buf();
    path.set_file_name(self.name);
    path.set_extension(EXT_DATA);

    Ok(Box::new(FileSegmentBlockInputStream {
      phase: ExecPhase::UnInited,
      next_block_idx: 0,
      path: Arc::new(path),
      reader: None,
      index: self.index.clone(),
      column_types,
      column_names,
    }))
  }
}

impl BlockInputStream for FileSegmentBlockInputStream {
  fn open(&mut self) -> Result<()> {
    assert_eq!(ExecPhase::UnInited, self.phase);

    let path = self.path.clone();
    let reader = Some(File::open(&*(path.clone()))
      .chain_err(move || {
        let err_msg = format!("Failed to open file: \"{:?}\"", path);
        error!("{:?}", err_msg);
        err_msg
      })?);

    self.reader = reader;
    self.next_block_idx = 0;
    self.phase = ExecPhase::Opened;
    Ok(())
  }

  fn next(&mut self) -> Result<Block> {
    assert!((ExecPhase::Opened == self.phase) || (ExecPhase::Executed == self.phase));
    if self.next_block_idx >= self.index.get_block_node().len() {
      bail!(ErrorKind::DB(DBErrorKind::EOF))
    }

    let block_handle = self.index.get_block_node().get(self.next_block_idx).unwrap();

    for x in self.column_names.keys() {
      assert!(block_handle.get_column_node().contains_key(x),
              "Column {:?} doesn't exist in {:?}.", x, self.path);
    }

    let mut sorted_column_ids = self.column_names.keys().cloned().collect::<Vec<i32>>();
    sorted_column_ids.sort_by_key(|id| block_handle.column_node[id].start);

    let mut columns: Vec<ColumnWithInfo> = Vec::new();
    for column_id in &sorted_column_ids {
      let column_handle = block_handle.get_column_node().get(column_id).unwrap();
      let column = box self.load_column(*column_id, &column_handle)?;
//      let column_start = column_handle.get_start() as u64;
//      let mut column_factory = self.column_factories.get_mut(column_id).unwrap();
//      //TODO: Optimize this
//      let buf_len = (column_handle.get_end() - column_handle.get_start()) as usize;
//      let mut buf = Vec::with_capacity(buf_len);
//      unsafe {
//        buf.set_len(buf_len);
//      }
//
//      let file_ref: &mut File = self.file.as_mut().unwrap();
//      file_ref.seek(SeekFrom::Start(column_start))?;
//      file_ref.read_exact(&mut buf)?;
//
//      let column = column_factory.create_column(&buf)?;
      columns.push(ColumnWithInfo {
        name: self.column_names.get(column_id).unwrap().clone(),
        id: Some(*column_id),
        column: CowPtr::Owned(column),
      });
    }

    self.next_block_idx += 1;

    Ok(Block {
      columns,
      eof: self.next_block_idx >= self.index.get_block_node().len(),
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

impl FileSegmentBlockInputStream {
  fn load_column(&mut self, column_id: i32, column_node: &ColumnNode) -> Result<ArrowColumn> {
    unimplemented!()
  }
}