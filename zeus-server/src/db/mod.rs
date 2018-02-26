pub mod column;
pub mod data_type;
mod simple_db;

use std::clone::Clone;
use std::boxed::Box;
use std::vec::Vec;
use std::fmt::Display;
use std::path::Path;

use db::column::Column;
use util::error::Result;
use util::cow_ptr::CowPtr;
use exec::Block;
use self::simple_db::SimpleDB;


#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct ScanContext {
    pub db_id: i32,
    pub table_id: i32,
    pub column_ids: Vec<i32>
}

pub trait DB {
    fn scan(&self, scan_context: &ScanContext) -> Result<Box<BlockInputStream>>;
    fn close(&mut self) -> Result<()>;
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidHeader
}

pub fn open(config: &DBConfig) -> Result<Box<DB>> {
    Ok(Box::new(SimpleDB::new(config)?))
}

pub type DBResult = Result<i32>;


