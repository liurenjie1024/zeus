use std::path::Path;
use std::path::PathBuf;
use std::borrow::ToOwned;
use std::fs::File;
use std::io::Read;
use std::io::SeekFrom;
use std::io::Seek;
use std::sync::Arc;
use std::io::Error as StdIoError;
use std::io::ErrorKind as StdIoErrorKind;
use std::collections::HashMap;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian;
use protobuf::parse_from_reader;

use exec::ExecPhase;
use exec::Block;
use storage::column::Column;
use storage::column::vec_column_data::VecColumnData;
use storage::ScanContext;
use storage::BlockInputStream;
use storage::ErrorKind as DBErrorKind;
use rpc::zeus_meta::ColumnType;
use rpc::zeus_blizard_format::SegmentIndex;
use rpc::zeus_blizard_format::ColumnNode;
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
    path.push(name);

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
      bail!(StdIoError::new(StdIoErrorKind::NotFound, err_msg));
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

    let mut path = self.data_path.to_path_buf();
    path.set_file_name(&self.name);
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

    let mut columns: Vec<Column> = Vec::new();
    for column_id in &sorted_column_ids {
      let column_handle = block_handle.get_column_node().get(column_id).unwrap();
      let column = FileSegmentBlockInputStream::load_column(
        &mut self.reader.as_mut().unwrap(),
        &column_handle,
        self.column_types[column_id],
        block_handle.block_column_size as usize)?;
      columns.push(column);
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

macro_rules! create_vector {
  ($reader: ident, $dest_type: ident, $endian: ty, $len: expr) => {
    {
      let mut buf = Vec::with_capacity($len);
      for _ in 0..$len {
        let v = ReadBytesExt::$dest_type::<$endian>($reader).
          chain_err(|| {
            let err_msg = "Failed to read vector data.";
            error!("{}", err_msg);
            err_msg
          })?;
        buf.push(v);
      }
      buf
    }
  };
  ($reader: ident, $dest_type: ident, $len: expr) => {
    {
      let mut buf = Vec::with_capacity($len);
      for _ in 0..$len {
        let v = ReadBytesExt::$dest_type($reader).
          chain_err(|| {
            let err_msg = "Failed to read vector data.";
            error!("{}", err_msg);
            err_msg
          })?;
        buf.push(v);
      }
      buf
    }
  };
}

impl FileSegmentBlockInputStream {
  /// We required that data stored is little endian.
  fn load_column(reader: &mut File,
                 column_node: &ColumnNode,
                 column_type: ColumnType,
                 column_len: usize) -> Result<Column> {
    assert!(column_len > 0);

    let start = column_node.get_start() as u64;
        
    reader.seek(SeekFrom::Start(start))
      .chain_err(|| {
        let err_msg = format!("Failed to seek to {}", start);
        error!("{}", err_msg);
        err_msg
      })?;

    let column = match column_type {
      ColumnType::BOOL => {
        let ret = create_vector!(reader, read_u8, column_len)
          .into_iter()
          .map(|x| x>0)
          .collect::<Vec<bool>>();

        Column::new_vec(column_type, VecColumnData::from(ret))
      },
      ColumnType::INT8 => {
        let ret = create_vector!(reader, read_i8, column_len);
        Column::new_vec(column_type, VecColumnData::from(ret))
      },
      ColumnType::INT16 => {
        let ret = create_vector!(reader, read_i16, LittleEndian, column_len);
        Column::new_vec(column_type, VecColumnData::from(ret))
      },
      ColumnType::INT32 => {
        let ret = create_vector!(reader, read_i32, LittleEndian, column_len);
        Column::new_vec(column_type, VecColumnData::from(ret))
      },
      ColumnType::INT64 => {
        let ret = create_vector!(reader, read_i64, LittleEndian, column_len);
        Column::new_vec(column_type, VecColumnData::from(ret))
      },
      ColumnType::FLOAT4 => {
        let ret = create_vector!(reader, read_f32, LittleEndian, column_len);
        Column::new_vec(column_type, VecColumnData::from(ret))
      },
      ColumnType::FLOAT8 => {
        let ret = create_vector!(reader, read_f64, LittleEndian, column_len);
        Column::new_vec(column_type, VecColumnData::from(ret))
      },
      ColumnType::TIMESTAMP => {
        let ret = create_vector!(reader, read_i64, LittleEndian, column_len);
        Column::new_vec(column_type, VecColumnData::from(ret))
      },
      ColumnType::STRING => {
        let offsets = create_vector!(reader, read_i32, LittleEndian, column_len + 1)
          .into_iter()
          .map(|s| s as usize)
          .collect::<Vec<usize>>();

        let mut strs: Vec<String> = Vec::new();
        for i in 1..offsets.len() {
          let total_string_size = offsets[i] - offsets[i-1];
          let mut chars = Vec::with_capacity(total_string_size);
          chars.resize(total_string_size, 0 as u8);
          reader.read_exact(&mut chars)
            .chain_err(|| {
              let err_msg = "Failed to read string.";
              error!("{}", err_msg);
              err_msg
            })?;
          
          let utf8 = String::from_utf8(chars)
            .chain_err(|| {
              let  err_msg = "Failed to parse string";
              error!("{}", err_msg);
              err_msg
            })?;
          strs.push(utf8)
        }

        Column::new_vec(column_type, VecColumnData::from(strs))
      }
    };

    Ok(column)
  }
}

