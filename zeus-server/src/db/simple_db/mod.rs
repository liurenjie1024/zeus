mod simple_file_segment;
mod simple_column_factory;

use std::collections::HashMap;
use std::collections::LinkedList;
use std::sync::Arc;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use rpc::zeus_meta::ZeusDBSchema;
use protobuf::core::parse_from_reader;

use db::DB;
use db::DBConfig;
use util::error::Result;
use db::ScanContext;
use db::BlockInputStream;
use self::simple_file_segment::SimpleFileSegment;

const SCHEMA_FILE_NAME: &'static str = "schema.pb";
const TABLE_PLAYLIST_FILE: &'static str = "table.pl";

pub struct SimpleDB {
    schema: Arc<ZeusDBSchema>,
    config: Arc<DBConfig>,
    tables: HashMap<i32, Arc<SimpleTable>>
}

pub struct SimpleDBContext {
    pub schema: Arc<ZeusDBSchema>,
    pub config: Arc<DBConfig>
}

impl SimpleDB {
    pub fn new(config: &DBConfig) -> Result<SimpleDB> {
        info!("Trying to load database from {}", config.path);

        let mut schema_file_path = PathBuf::from(config.path.clone());
        schema_file_path.push(SCHEMA_FILE_NAME);
        let schema_file = File::open(schema_file_path)?;
        let mut schema_reader = BufReader::new(schema_file);
        let schema = parse_from_reader::<ZeusDBSchema>(&mut schema_reader)?;
        info!("Read db schema!");

        let mut tables = HashMap::new();
        for table_id in schema.get_tables().keys() {
            let native_table = SimpleTable::new(config, *table_id)?;
            tables.insert(*table_id, Arc::new(native_table));
        }

        Ok(SimpleDB {
            schema: Arc::new(schema),
            config: Arc::new(config.clone()),
            tables
        })
    }

    fn get_context(&self) -> SimpleDBContext {
        SimpleDBContext {
            schema: self.schema.clone(),
            config: self.config.clone()
        }
    }
}

impl DB for SimpleDB {
    fn scan(&self, scan_context: &ScanContext) -> Result<Box<BlockInputStream>> {
        unimplemented!()
    }

    fn close(&mut self) -> Result<()> {
        unimplemented!()
    }
}

struct SimpleTable {
    table_id: i32,
    file_segments: LinkedList<SimpleFileSegment>
}

impl SimpleTable {
    pub fn new(config: &DBConfig, table_id: i32) -> Result<SimpleTable> {
        let mut playlist_path = PathBuf::from(config.path.clone());
        playlist_path.push(table_id.to_string());
        playlist_path.push(TABLE_PLAYLIST_FILE);


        let playlist_file = File::open(playlist_path)?;
        let mut playlist_file  = BufReader::new(playlist_file);

        let mut segments = LinkedList::new();

        loop {
            let mut line = "".to_string();
            let result = playlist_file.read_line(&mut line)?;

            if result == 0 {
                break;
            }

            let mut seg_path = PathBuf::from(config.path.clone());
            seg_path.push(table_id.to_string());
            seg_path.push(line);

            let seg = SimpleFileSegment {
                table_id: table_id,
                path: seg_path.to_str().unwrap().to_string()
            };

            seg.validate()?;

            segments.push_back(seg);
        }


        Ok(SimpleTable {
            table_id: table_id,
            file_segments: segments
        })
    }
}



