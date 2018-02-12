use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;

use protobuf::core::parse_from_reader;

use rpc::zeus_meta::ZeusDBSchema;
use db::column::Column;
use db::DBConfig;
use util::error::Result;

static SCHEMA_FILE_NAME: &'static str = "schema.pb";
static DATA_FILE_SUFFIX: &'static str = ".data";

struct MemDB {
    schema: ZeusDBSchema,
    tables: HashMap<i32, Box<MemTable>>
}

struct MemTable {
    id: i32,
    columns: HashMap<i32, Box<Column>>
}

impl MemDB {
    fn new(config: &DBConfig) -> Result<Box<MemDB>> {
        debug!("Trying to load database from {}", config.path);

        let mut schema_file_path = PathBuf::from(config.path);
        schema_file_path.push(SCHEMA_FILE_NAME);

        let schema_file = File::open(schema_file_path)?;
        let mut schema_reader = BufReader::from(schema_file);
        let schema = parse_from_reader::<ZeusDBSchema>(&schema_reader)?;


    }

    fn load_table(config: &DBConfig, db_schema: &ZeusDBSchema, table_id: i32) -> Result<Box<MemTable>> {

    }
}

