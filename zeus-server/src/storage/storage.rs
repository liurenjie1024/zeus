use std::sync::Arc;

use util::error::Result;
use rpc::zeus_data::ScanNode;
use storage::BlockInputStream;
use storage::CatalogManager;

pub struct ScanContext<'a> {
  pub scan_node: &'a ScanNode,
  pub catalog_manager: Arc<CatalogManager>,
}

pub trait Storage: Send + Sync {
  fn scan(
    &self,
    scan_context: &ScanContext,
  ) -> Result<Box<BlockInputStream>>;
}
