pub mod server;
pub mod config;
pub mod data_service;
mod meta_service;

use std::sync::Arc;

use storage::StorageManager;
use storage::CatalogManager;
use scheduler::ExecutorService;
use util::error::Result;


pub const MAX_GRPC_RECV_MSG_SIZE: usize = 10*1024*1024;
pub const MAX_GRPC_SEND_MSG_SIZE: usize = 10*1024*1024;

/// A container for runtime components.
#[derive(Clone)]
pub struct ServerContext {
    storage_manager: Arc<StorageManager>,
    catalog_manager: Arc<CatalogManager>,
    query_scheduler: Arc<ExecutorService>
}

impl ServerContext {
    pub fn new(storage_manager: Arc<StorageManager>,
               catalog_manager: Arc<CatalogManager>,
               query_scheduler: Arc<ExecutorService>) -> ServerContext {
        ServerContext {
            storage_manager,
            catalog_manager,
            query_scheduler
        }
    }

    pub fn get_storage_manager(&self) -> Arc<StorageManager> {
        self.storage_manager.clone()
    }

    pub fn get_catalog_manager(&self) -> Arc<CatalogManager> {
        self.catalog_manager.clone()
    }
}

