pub mod column;
pub mod schema;
pub mod table;
pub mod db;
pub mod segment;
pub mod data_type;

pub struct DBConfig {
    // pending appendable segment num per table
    pub max_pending_segment_num: usize,
    // maximum size of appendable segment
    pub max_appendable_segment_size: usize
}

pub enum DBErr {
    /// Table not found err, it contains table id.
    TableNotFound(i32)
}

pub type DBResult = Result<i32, DBErr>;
