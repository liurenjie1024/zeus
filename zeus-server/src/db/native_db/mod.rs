mod native_file_segment;

use rpc::zeus_meta::ZeusDBSchema;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::sync::Arc;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use db::DBConfig;
use util::error::Result;
use self::native_file_segment::NativeFileSegment;

const SCHEMA_FILE: &'static str = "schema.pb";
const TABLE_PLAYLIST_FILE: &'static str = "table.pl";

struct NativeDB {
    schema: ZeusDBSchema,
    config: DBConfig,
    tables: HashMap<i32, Arc<NativeTable>>
}

impl NativeDB {
    pub fn new(config: &DBConfig) -> Result<NativeDB> {
        
    }
}

struct  NativeTable {
    table_id: i32,
    file_segments: LinkedList<NativeFileSegment>
}

impl NativeTable {
    pub fn new(config: &DBConfig, table_id: i32) -> Result<NativeTable> {
        let mut playlist_path = PathBuf::from(config.path);
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

            let mut seg_path = PathBuf::from(config.path);
            seg_path.push(table_id.to_string());
            seg_path.push(line);

            let seg = NativeFileSegment {
                path: seg_path
            };

            seg.validate()?;

            segments.push_back(seg);
        }


        Ok(NativeTable {
            table_id: table_id,
            file_segments: segments
        })
    }
}



