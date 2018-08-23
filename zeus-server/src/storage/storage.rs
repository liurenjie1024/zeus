use std::sync::Arc;

use util::errors::*;
use rpc::zeus_plan::ScanNode;
use storage::BlockInputStream;
use catalog::CatalogManager;
use exec2::exec::ExecNode as ExecNode2;

pub struct ScanContext<'a> {
  pub scan_node: &'a ScanNode,
  pub catalog_manager: Arc<CatalogManager>,
}

pub trait Storage: Send + Sync {
  fn get_id(&self) -> i32;
  fn scan(
    &self,
    scan_context: &ScanContext,
  ) -> Result<Box<BlockInputStream>>;
  fn scan2(&self, scan_context: &ScanContext) -> Result<Box<ExecNode2>>;
  fn get_row_count(&self) -> Result<i64>;
}
