use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;

use protobuf::core::parse_from_reader;
use serde::ser::Serialize;
use serde::de::Deserialize;

use rpc::zeus_meta::ZeusDBSchema;
use db::column::Column;
use db::DBConfig;
use util::error::Result;

static SCHEMA_FILE_NAME: &'static str = "schema.pb";
static MEAT_FILE_NAME: &'static str = "meta.json";
static DATA_FILE_SUFFIX: &'static str = ".data";

struct ColumnMeta {
    /// start offset in table data file
    start: usize,
    /// end offset in table data file
    end: usize,
}

struct TableMeta {
    column_meta: HashMap<i32, ColumnMeta>
}

#[derive(Serialize, Deserialize, Debug)]
struct DBMeta {
    table_meta: HashMap<i32, TableMeta>
}

struct MemDB {
    schema: ZeusDBSchema,
    db_meta: DBMeta,
    tables: HashMap<i32, Box<MemTable>>,
}

struct MemTable {
    id: i32,
    columns: HashMap<i32, Box<Column>>,
}

impl MemDB {
    fn new(config: &DBConfig) -> Result<Box<MemDB>> {
        debug!("Trying to load database from {}", config.path);

        let mut schema_file_path = PathBuf::from(config.path);
        schema_file_path.push(SCHEMA_FILE_NAME);
        let schema_file = File::open(schema_file_path)?;
        let mut schema_reader = BufReader::from(schema_file);
        let schema = parse_from_reader::<ZeusDBSchema>(&schema_reader)?;
        info!("Read db schema!");

        let mut meta_file_path = PathBuf::from(config.path);
        meta_file_path.push(MEAT_FILE_NAME);
        let meta_file = File::open(meta_file_path)?;
        let mut meta_reader = BufReader::from(meta_file);
        let meta = serde_json::from_reader::<DBMeta>(meta_reader)?;
        info!("Read db meta!")
    }

    fn load_table(db_meta: &DBMeta, table_id: i32, path: PathBuf)
                  -> Result<Box<MemTable>> {

    }
}
