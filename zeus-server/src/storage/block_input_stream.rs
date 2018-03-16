use std::vec::Vec;

use util::error::Result;
use util::error::Error::DBError;
use storage::ErrorKind as DBErrorKind;
use exec::Block;

pub trait BlockInputStream: Send + 'static {
  fn open(&mut self) -> Result<()>;
  fn next(&mut self) -> Result<Block>;
  fn close(&mut self) -> Result<()>;
}

pub struct CombinedBlockInputStream {
  streams: Vec<Box<BlockInputStream>>,
  idx: usize,
  need_open: bool,
}

impl CombinedBlockInputStream {
  pub fn new(streams: Vec<Box<BlockInputStream>>) -> CombinedBlockInputStream {
    CombinedBlockInputStream {
      streams,
      idx: 0,
      need_open: true,
    }
  }
}

impl BlockInputStream for CombinedBlockInputStream {
  fn open(&mut self) -> Result<()> {
    self.idx = 0;
    self.need_open = true;

    Ok(())
  }

  fn next(&mut self) -> Result<Block> {
    if self.idx >= self.streams.len() {
      return Err(DBError(DBErrorKind::EOF));
    }

    let is_last_stream = (self.idx == (self.streams.len() - 1));
    let cur_stream = self.streams.get_mut(self.idx).unwrap();
    if self.need_open {
      cur_stream.open()?;
      self.need_open = false;
    }

    let mut block = cur_stream.next()?;

    if block.eof {
      self.idx += 1;
      self.need_open = true;
      cur_stream.close();
    }

    if !block.eof || !is_last_stream {
      block.eof = false;
    }

    Ok(block)
  }

  fn close(&mut self) -> Result<()> {
    Ok(())
  }
}
