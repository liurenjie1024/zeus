use std::path::Path;
use std::io::ErrorKind;
use std::io::Error as StdIoError;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use db::ScanContext;
use db::BlockInputStream;
use exec::Block;
use rpc::zeus_simple_format::BlockHandles;
use util::error::Result;
use util::error::Error;

pub struct SimpleFileSegment {
    pub path: String
}

struct FileSegmentBlockInputStream {
    file: File,
    blocks: BlockHandles
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
        unimplemented!()
    }
}

impl BlockInputStream for FileSegmentBlockInputStream {
    fn open(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn next(&mut self) -> Result<Block> {
        unimplemented!()
    }

    fn close(&mut self) -> Result<()> {
        unimplemented!()
    }
}


