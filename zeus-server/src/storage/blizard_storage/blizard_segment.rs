use std::path::Path;
use std::path::PathBuf;
use std::vec::Vec;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::sync::Arc;

use parquet::file::reader::FileReader as ParquetFileReader;
use parquet::file::reader::SerializedFileReader as ParquetSerializedFileReader;
use parquet::file::reader::RowGroupReader;
use parquet::file::metadata::RowGroupMetaDataPtr;
use parquet::column::reader::ColumnReader;

use exec::ExecPhase;
use exec::Block;
use exec::expression::Expr;
use storage::column::Column;
use storage::column::vec_column_data::Datum;
use storage::column::ColumnBuilder;
use storage::ScanContext;
use storage::BlockInputStream;
use storage::ErrorKind as DBErrorKind;
use rpc::zeus_meta::ColumnType;
use util::errors::*;

#[derive(Clone, Debug)]
pub(super) struct BlizardSegment {
  data_path: Arc<PathBuf>,
}

impl BlizardSegment {
  pub fn open<P: AsRef<Path>>(path: &P) -> Result<BlizardSegment> {
    Ok(BlizardSegment {
      data_path: Arc::new(path.as_ref().to_path_buf()),
    })
  }

  pub fn get_path(&self) -> &Path {
    self.data_path.as_path()
  }
}

struct FileSegmentBlockInputStream {
  phase: ExecPhase,
  next_block_idx: usize,
  reader: Box<dyn ParquetFileReader>,
  filter: Option<Expr>,
  column_types: HashMap<String, ColumnType>,
}

impl BlizardSegment {
  pub fn scan(
    &self,
    context: &ScanContext,
  ) -> Result<Box<BlockInputStream>>
  {
    let scan_node = context.scan_node;

    // Create filter
    // Currently only one filter is supported
    if scan_node.get_filters().len() > 1 {
      bail!("Scan node can only have one filter");
    }

    let filter = scan_node.get_filters()
      .first()
      .map(Expr::new)
      .map_or(Ok(None), |e| e.map(Some))?;


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

    debug!("Num of row groups for {:?} is {:?}", self.data_path, reader.num_row_groups());

    Ok(Box::new(FileSegmentBlockInputStream {
      phase: ExecPhase::UnInited,
      next_block_idx: 0,
      reader,
      filter,
      column_types,
    }))
  }

  pub fn get_row_count(&self) -> Result<i64> {
    let reader: Box<dyn ParquetFileReader> = box {
      ParquetSerializedFileReader::try_from(self.data_path.as_path())?
    };

    Ok(reader.metadata().file_metadata().num_rows())
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
    let row_groups_count = self.reader.num_row_groups();
    if self.next_block_idx >= row_groups_count {
      bail!(ErrorKind::DB(DBErrorKind::EOF))
    }

    let mut result: Option<Block> = None;

    while self.next_block_idx < row_groups_count {
      let row_group_reader = self.reader.get_row_group(self.next_block_idx)?;
      let row_num = row_group_reader.metadata().num_rows();
      debug!("Row number for row group {:?} is {:?}", self.next_block_idx, row_num);

      let row_group_metadata = row_group_reader.metadata();

      let need_scan = self.filter.as_ref()
        .map(|x| x.filter(row_group_metadata.clone()))
        .unwrap_or(true);

      self.next_block_idx += 1;

      if need_scan {
        let mut block = self.read_next_block(row_group_reader, row_group_metadata.clone())?;
        block.eof = self.next_block_idx >= self.reader.num_row_groups();

        result = Some(block);
        break;
      } else {
        debug!("Skipping row group {:?}", self.next_block_idx);
      }
    }

    Ok(result.unwrap_or(Block::default()))
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
  fn read_next_block(&self, row_group_reader: Box<dyn RowGroupReader>,
    row_group_metadata: RowGroupMetaDataPtr) -> Result<Block> {

    let row_num = row_group_reader.metadata().num_rows();
    let columns = row_group_metadata
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

    Ok(Block {
      columns,
      eof: false
    })
  }
}

impl FileSegmentBlockInputStream {
  fn build_column(row_num: usize,
    column_type: ColumnType,
    column_reader: ColumnReader) -> Result<Vec<Datum>> {
    match (column_type, column_reader) {
      (ColumnType::BOOL, ColumnReader::BoolColumnReader(mut r)) => {
        let mut vec = vec![false; row_num];
        let (num_read, _) = r.read_batch(row_num, None, None, &mut vec)?;
        debug!("Read {} rows.", num_read);
        Ok(vec.into_iter().map(|x| x.into()).collect())
      }
      (ColumnType::INT8, ColumnReader::Int32ColumnReader(mut r)) => {
        let mut vec = vec![0; row_num];
        let (num_read, _) = r.read_batch(row_num, None, None, &mut vec)?;
        debug!("Read {} rows.", num_read);
        Ok(vec.into_iter().map(|x| (x as i8).into()).collect())
      }
      (ColumnType::INT16, ColumnReader::Int32ColumnReader(mut r)) => {
        let mut vec = vec![0; row_num];
        let (num_read, _) = r.read_batch(row_num, None, None, &mut vec)?;
        debug!("Read {} rows.", num_read);
        Ok(vec.into_iter().map(|x| (x as i16).into()).collect())
      }
      (ColumnType::INT32, ColumnReader::Int32ColumnReader(mut r)) => {
        let mut vec = vec![0; row_num];
        let (num_read, _) = r.read_batch(row_num, None, None, &mut vec)?;
        debug!("Read {} rows.", num_read);
        Ok(vec.into_iter().map(|x| x.into()).collect())
      }
      (ColumnType::INT64, ColumnReader::Int64ColumnReader(mut r)) => {
        let mut vec = vec![0; row_num];
        let (num_read, _) = r.read_batch(row_num, None, None, &mut vec)?;
        debug!("Read {} rows.", num_read);
        Ok(vec.into_iter().map(|x| x.into()).collect())
      }
      (ColumnType::FLOAT4, ColumnReader::FloatColumnReader(mut r)) => {
        let mut vec = vec![0f32; row_num];
        let (num_read, _) = r.read_batch(row_num, None, None, &mut vec)?;
        debug!("Read {} rows.", num_read);
        Ok(vec.into_iter().map(|x| x.into()).collect())
      }
      (ColumnType::FLOAT8, ColumnReader::DoubleColumnReader(mut r)) => {
        let mut vec = vec![0f64; row_num];
        let (num_read, _) = r.read_batch(row_num, None, None, &mut vec)?;
        debug!("Read {} rows.", num_read);
        Ok(vec.into_iter().map(|x| x.into()).collect())
      }
      (ColumnType::STRING, ColumnReader::ByteArrayColumnReader(mut r)) => {
        let mut vec = Vec::with_capacity(row_num);
        vec.resize_default(row_num);
        let (num_read, _) = r.read_batch(row_num, None, None, &mut vec)?;
        debug!("Read {} rows.", num_read);
        vec.into_iter()
          .try_fold(Vec::new(), |mut ret, byte_arr| -> Result<Vec<Datum>> {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(byte_arr.data());
            let str = String::from_utf8(bytes)?;
            ret.push(str.into());
            Ok(ret)
          })
      }
      (column_type, _) => bail!("Unable to read column for {:?}", column_type)
    }
  }
}

