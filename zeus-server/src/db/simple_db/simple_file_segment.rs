use std::path::Path;
use std::io::ErrorKind;
use std::io::Error as StdIoError;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::collections::HashSet;

use bytes::{ByteOrder, BigEndian};
use protobuf::parse_from_bytes;

use db::ScanContext;
use db::BlockInputStream;
use exec::Block;
use exec::ExecPhase;
use rpc::zeus_simple_format::{BlockHandles, BlockHandle};
use util::error::Result;
use util::error::Error;
use db::ErrorKind as DBErrorKind;

pub struct SimpleFileSegment {
    pub path: String
}

struct FileSegmentBlockInputStream {
    phase: ExecPhase,
    next_block_idx: usize,
    path: String,
    file: File,
    blocks: BlockHandles,
    column_ids: HashSet<i32>
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

    pub fn scan(&self, context: &ScanContext) -> Result<Box<BlockInputStream>> {
        let file = File::open(&self.path)?;
        let blocks = BlockHandles::new();
        let column_ids = context.column_ids.clone();

        Ok(Box::new(FileSegmentBlockInputStream {
            phase: ExecPhase::UnInited,
            next_block_idx: 0,
            path: self.path.clone(),
            file,
            blocks,
            column_ids: column_ids.into_iter().collect()
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




