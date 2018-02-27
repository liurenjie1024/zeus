use std::path::Path;
use std::io::ErrorKind;
use std::io::Error as StdIoError;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::collections::HashSet;
use std::collections::HashMap;

use bytes::{ByteOrder, BigEndian};
use protobuf::parse_from_bytes;

use db::ScanContext;
use db::simple_db::SimpleDBContext;
use db::BlockInputStream;
use db::column::ColumnFactory;
use exec::Block;
use exec::ExecPhase;
use rpc::zeus_simple_format::{BlockHandles, BlockHandle};
use rpc::zeus_meta::FieldType;
use util::error::Result;
use util::error::Error;
use db::ErrorKind as DBErrorKind;
use super::simple_column_factory::create_column_factory;

pub struct SimpleFileSegment {
    pub table_id: i32,
    pub path: String
}

struct FileSegmentBlockInputStream {
    phase: ExecPhase,
    next_block_idx: usize,
    path: String,
    file: File,
    blocks: BlockHandles,
    column_types: HashMap<i32, FieldType>,
    column_factories: HashMap<i32, Box<ColumnFactory>>
}

impl SimpleFileSegment {
    pub fn validate(&self) -> Result<()> {
        let path = Path::new(&self.path);

        if path.exists() && path.is_file() {
            Ok(())
        } else {
            let msg = format!("{} not found", self.path);
            Err(Error::IoError(StdIoError::new(ErrorKind::NotFound, msg)))
        }
    }

    pub fn scan(&self, context: &ScanContext, db_context: &SimpleDBContext) -> Result<Box<BlockInputStream>> {
        let file = File::open(&self.path)?;
        let blocks = BlockHandles::new();

        let mut column_types: HashMap<i32, FieldType> = HashMap::new();

        let table_schema = db_context.schema.get_tables().get(&context.table_id).unwrap();
        for column_id in &context.column_ids {
            let column_schema = table_schema.get_fields().get(column_id).unwrap();
            column_types.insert(*column_id, column_schema.get_field_type());
        }

        let column_ids = context.column_ids.clone();

        Ok(Box::new(FileSegmentBlockInputStream {
            phase: ExecPhase::UnInited,
            next_block_idx: 0,
            path: self.path.clone(),
            file,
            blocks,
            column_types,
            column_factories: HashMap::new()
        }))
    }
}

impl BlockInputStream for FileSegmentBlockInputStream {
    fn open(&mut self) -> Result<()> {
        assert_eq!(ExecPhase::UnInited, self.phase);

        const MAGIC_NUM: [u8;4] = [0xAA, 0xBB, 0xCC, 0xDD];
        const VERSION: [u8;4] = [00, 00, 00, 01];

        let mut header = [0 as u8;8];
        self.file.read_exact(&mut header)?;

        if &header[0..4] != MAGIC_NUM || &header[4..] != VERSION {
            return Err(Error::DBError(DBErrorKind::InvalidHeader));
        }

        let mut index_len_buf = [0 as u8;4];
        self.file.seek(SeekFrom::End(4))?;
        self.file.read_exact(&mut index_len_buf)?;
        let index_len = BigEndian::read_u64(&index_len_buf) as usize;

        let mut block_handles_buf:Vec<u8> = Vec::with_capacity(index_len);
        self.file.seek(SeekFrom::Current((index_len as i64)*-1))?;
        self.file.read_exact(&mut block_handles_buf)?;

        let block_handles = parse_from_bytes::<BlockHandles>(&block_handles_buf)?;

        self.blocks = block_handles;

        for (column_id, column_type) in self.column_types.iter() {
            self.column_factories.insert(*column_id, create_column_factory(*column_type, self.blocks.get_block_column_size() as usize));
        }

        self.phase = ExecPhase::Opened;
        Ok(())
    }

    fn next(&mut self) -> Result<Block> {
//        assert!((ExecPhase::Opened == self.phase) || (ExecPhase::Executed == self.phase));
//
//        let block_handle = self.blocks.get_handles().get(self.next_block_idx).unwrap();
//
//        for x in &self.column_ids {
//            assert!(block_handle.columns.contains_key(x), "Column {} doesn't exist in {}.", x, self.path);
//        }
//
//        let mut sorted_column_ids = self.column_ids.clone()
//            .into_iter()
//            .collect::<Vec<i32>>();
//        sorted_column_ids.sort_by_key(|id| block_handle.columns[x].end);
//
//        panic!("not implemented!")
        unimplemented!()

    }

    fn close(&mut self) -> Result<()> {
        unimplemented!()
    }
}




