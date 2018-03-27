pub mod catalog_manager;
pub mod db_schema;
pub mod table_schema;
pub mod column_schema;

pub use self::catalog_manager::CatalogManager as CatalogManager;
pub use self::table_schema::TableSchema as TableSchema;
pub use self::column_schema::ColumnSchema as ColumnSchema;

use std::sync::Arc;
use std::path::PathBuf;
use std::fs::File;

use protobuf::parse_from_reader;

use server::config::ZeusConfig;
use util::error::Result;

pub fn load(config: &ZeusConfig) -> Result<Arc<CatalogManager>> {
  let mut schema_file_path = PathBuf::from(&config.storage.root_path);
  schema_file_path.push(&config.storage.schema_filename);

  let mut file = File::open(&schema_file_path)?;

  let zeus_catalog = parse_from_reader(&mut file)?;

  Ok(Arc::new(CatalogManager::new(zeus_catalog)))
}

