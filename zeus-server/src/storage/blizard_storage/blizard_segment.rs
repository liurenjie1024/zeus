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
use std::vec::Vec;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian;
use parquet::file::reader::FileReader as ParquetFileReader;
use parquet::file::reader::SerializedFileReader as ParquetSerializedFileReader;
use parquet::column::reader::ColumnReader;
use protobuf::parse_from_reader;

use exec::ExecPhase;
use exec::Block;
use storage::column::Column;
use storage::column::vec_column_data::Datum;
use storage::column::ColumnBuilder;
use storage::ScanContext;
use storage::BlockInputStream;
use storage::ErrorKind as DBErrorKind;
use rpc::zeus_meta::ColumnType;
use rpc::zeus_blizard_format::SegmentIndex;
use rpc::zeus_blizard_format::ColumnNode;
use util::errors::*;

pub(super) struct BlizardSegment {
  data_path: PathBuf,
}

impl BlizardSegment {
  pub fn open<P: AsRef<Path>>(path: &P) -> Result<BlizardSegment> {
    Ok(BlizardSegment {
      data_path: path.as_ref().to_path_buf(),
    })
  }
}

struct FileSegmentBlockInputStream {
  phase: ExecPhase,
  next_block_idx: usize,
  reader: Box<dyn ParquetFileReader>,
  column_types: HashMap<String, ColumnType>,
}

impl BlizardSegment {
  pub fn scan(
    &self,
    context: &ScanContext,
  ) -> Result<Box<BlockInputStream>>
  {
    let scan_node = context.scan_node;

    let mut column_types: HashMap<String, ColumnType> = HashMap::new();

    let table_schema =
      context.catalog_manager.get_table_schema(scan_node.table_id).unwrap();
    for column_id in &scan_node.columns {
      let column_schema = table_schema.get_column_schema(*column_id).unwrap();
      column_types.insert(column_schema.get_name(), column_schema.get_type());
    }

    let reader: Box<dyn ParquetFileReader> = box {
      ParquetSerializedFileReader::try_from(self.data_path.as_path())?
    };

    Ok(Box::new(FileSegmentBlockInputStream {
      phase: ExecPhase::UnInited,
      next_block_idx: 0,
      reader,
      column_types,
    }))
  }
}

impl BlockInputStream for FileSegmentBlockInputStream {
  fn open(&mut self) -> Result<()> {
    assert_eq!(ExecPhase::UnInited, self.phase);

    self.next_block_idx = 0;
    self.phase = ExecPhase::Opened;
    Ok(())
  }

  fn next(&mut self) -> Result<Block> {
    assert!((ExecPhase::Opened == self.phase) || (ExecPhase::Executed == self.phase));
    if self.next_block_idx >= self.reader.num_row_groups() {
      bail!(ErrorKind::DB(DBErrorKind::EOF))
    }

    let row_group_reader = self.reader.get_row_group(self.next_block_idx)?;
    let row_num = row_group_reader.metadata().num_rows();

    let column_vec = row_group_reader.metadata()
      .columns()
      .iter()
      .enumerate()
      .filter(|c| self.column_types.contains_key(&c.1.column_path().string()))
      .try_fold(Vec::new(), |mut columns, c| -> Result<Vec<Column>> {
        let column_name = c.1.column_path().string();
        let column_reader = row_group_reader.get_column_reader(c.0)?;
        let column_type = self.column_types[&column_name];

        let data_vec = FileSegmentBlockInputStream::build_column(row_num as usize,
          column_type, column_reader)?;
        let column = ColumnBuilder::new_vec(column_type, data_vec)
          .set_name(column_name.as_str())
          .build();

        columns.push(column);
        Ok(columns)
      })?;

    self.next_block_idx += 1;

    Ok(Block {
      columns: column_vec,
      eof: self.next_block_idx >= self.reader.num_row_groups(),
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
  fn build_column(row_num: usize,
    column_type: ColumnType,
    column_reader: ColumnReader) -> Result<Vec<Datum>> {
    match (column_type, column_reader) {
      (ColumnType::BOOL, ColumnReader::BoolColumnReader(mut r)) => {
        let mut vec = Vec::<bool>::with_capacity(row_num);
        r.read_batch(row_num, None, None, &mut vec)?;
        Ok(vec.into_iter().map(|x| x.into()).collect())
      }
      (ColumnType::INT8, ColumnReader::Int32ColumnReader(mut r)) => {
        let mut vec = Vec::<i32>::with_capacity(row_num);
        r.read_batch(row_num, None, None, &mut vec)?;
        Ok(vec.into_iter().map(|x| (x as i8).into()).collect())
      }
      (ColumnType::INT16, ColumnReader::Int32ColumnReader(mut r)) => {
        let mut vec = Vec::<i32>::with_capacity(row_num);
        r.read_batch(row_num, None, None, &mut vec)?;
        Ok(vec.into_iter().map(|x| (x as i16).into()).collect())
      }
      (ColumnType::INT32, ColumnReader::Int32ColumnReader(mut r)) => {
        let mut vec = Vec::<i32>::with_capacity(row_num);
        r.read_batch(row_num, None, None, &mut vec)?;
        Ok(vec.into_iter().map(|x| x.into()).collect())
      }
      (ColumnType::FLOAT4, ColumnReader::FloatColumnReader(mut r)) => {
        let mut vec = Vec::<f32>::with_capacity(row_num);
        r.read_batch(row_num, None, None, &mut vec)?;
        Ok(vec.into_iter().map(|x| x.into()).collect())
      }
      (ColumnType::FLOAT8, ColumnReader::DoubleColumnReader(mut r)) => {
        let mut vec = Vec::<f64>::with_capacity(row_num);
        r.read_batch(row_num, None, None, &mut vec)?;
        Ok(vec.into_iter().map(|x| x.into()).collect())
      }
      (column_type, _) => bail!("Unable to read column for {:?}", column_type)
    }
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
                 column_len: usize) -> Result<Vec<Datum>> {
    assert!(column_len > 0);

    let start = column_node.get_start() as u64;
        
    reader.seek(SeekFrom::Start(start))
      .chain_err(|| {
        let err_msg = format!("Failed to seek to {}", start);
        error!("{}", err_msg);
        err_msg
      })?;

    let column_vec = match column_type {
      ColumnType::BOOL => {
        create_vector!(reader, read_u8, column_len)
          .into_iter()
          .map(|x| x>0)
          .map(|x| x.into())
          .collect::<Vec<Datum>>()
      },
      ColumnType::INT8 => {
        create_vector!(reader, read_i8, column_len)
          .into_iter()
          .map(|x| x.into())
          .collect::<Vec<Datum>>()
      },
      ColumnType::INT16 => {
        create_vector!(reader, read_i16, LittleEndian, column_len)
          .into_iter()
          .map(|x| x.into())
          .collect::<Vec<Datum>>()
      },
      ColumnType::INT32 => {
        create_vector!(reader, read_i32, LittleEndian, column_len)
          .into_iter()
          .map(|x| x.into())
          .collect::<Vec<Datum>>()
      },
      ColumnType::INT64 => {
        create_vector!(reader, read_i64, LittleEndian, column_len)
          .into_iter()
          .map(|x| x.into())
          .collect::<Vec<Datum>>()
      },
      ColumnType::FLOAT4 => {
        create_vector!(reader, read_f32, LittleEndian, column_len)
          .into_iter()
          .map(|x| x.into())
          .collect::<Vec<Datum>>()
      },
      ColumnType::FLOAT8 => {
        create_vector!(reader, read_f64, LittleEndian, column_len)
          .into_iter()
          .map(|x| x.into())
          .collect::<Vec<Datum>>()
      },
      ColumnType::TIMESTAMP => {
        create_vector!(reader, read_i64, LittleEndian, column_len)
          .into_iter()
          .map(|x| x.into())
          .collect::<Vec<Datum>>()
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

        strs.into_iter()
          .map(|x| x.into())
          .collect::<Vec<Datum>>()
      }
    };

    Ok(column_vec)
  }
}

