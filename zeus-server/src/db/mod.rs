pub mod column;
pub mod schema;
pub mod table;
pub mod db;
pub mod segment;
pub mod data_type;
mod mem_db;
mod native_db;

use std::clone::Clone;
use std::boxed::Box;
use std::vec::Vec;
use std::fmt::Display;
use std::path::Path;

use util::error::Result;


#[derive(Copy, Clone, Debug, Display)]
pub struct DBConfig {
    // pending appendable segment num per table
    pub max_pending_segment_num: usize,
    // maximum size of appendable segment
    pub max_appendable_segment_size: usize,
    // database directory
    pub path: String
}

pub trait BlockInputStream {
    fn open(&mut self) -> Result<()>;
    fn next(&mut self) -> Result<Block>;
    fn close(&mut self) -> Result<()>;
}

pub struct ScanContext {
    pub db_id: i32,
    pub table_id: i32,
    pub column_ids: Vec<i32>
}

pub trait DB {
    fn scan(&self, scan_context: &ScanContext) -> Result<Box<BlockInputStream>>;
    fn close(&mut self) -> Result<()>;
}

pub fn open(config: &DBConfig) -> Result<Box<DB>> {
}

pub type DBResult = Result<i32>;


