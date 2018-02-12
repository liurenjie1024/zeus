
use util::cow_ptr::CowPtr;
use db::column::Column;
use super::Block;
use util::error::Result;

pub struct ScannerContext {
}

pub trait TableScanner {
    fn open(&mut self, context: &ScannerContext) -> Result<()>;
    fn next(&mut self) -> Result<Block>;
    fn close(&mut self) -> Result<()>;
}