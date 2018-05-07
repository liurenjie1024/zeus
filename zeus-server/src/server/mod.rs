pub mod config;
pub mod data_service;
mod meta_service;
pub mod server;

use std::sync::Arc;

use storage::StorageManager;
use catalog::CatalogManager;
use scheduler::ExecutorService;

pub const MAX_GRPC_RECV_MSG_SIZE: usize = 10 * 1024 * 1024;
pub const MAX_GRPC_SEND_MSG_SIZE: usize = 10 * 1024 * 1024;

/// A container for runtime components.
#[derive(Clone)]
pub struct ServerContext {
  storage_manager: Arc<StorageManager>,
  catalog_manager: Arc<CatalogManager>,
  query_scheduler: Arc<ExecutorService>,
}

impl ServerContext {
  pub fn new(
    storage_manager: Arc<StorageManager>,
    catalog_manager: Arc<CatalogManager>,
    query_scheduler: Arc<ExecutorService>,
  ) -> ServerContext
  {
    ServerContext {
      storage_manager,
      catalog_manager,
      query_scheduler,
    }
  }

  pub fn get_storage_manager(&self) -> Arc<StorageManager> {
    self.storage_manager.clone()
  }

  pub fn get_catalog_manager(&self) -> Arc<CatalogManager> {
    self.catalog_manager.clone()
  }
}

#[cfg(test)]
mod tests {
  use std::default::Default;
  use std::sync::Arc;

  use scheduler::ExecutorService;
  use scheduler::Task;
  use storage::StorageManager;
  use catalog::CatalogManager;
  use util::errors::*;
  use super::ServerContext;

  struct DefaultExecutorService {}

  impl ExecutorService for DefaultExecutorService {
    fn submit(&self, _task: Task) -> Result<()> {
      bail!("Not implemented!")
    }

    fn shutdown(&self) -> Result<usize> {
      bail!("Not implemented!")
    }
  }

  impl Default for ServerContext {
    fn default() -> Self {
      ServerContext {
        storage_manager: Arc::new(StorageManager::default()),
        catalog_manager: Arc::new(CatalogManager::default()),
        query_scheduler: Arc::new(DefaultExecutorService {})
      }
    }
  }
}
