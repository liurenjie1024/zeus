use std::sync::Arc;

use futures::BoxFuture;
use futures::IntoFuture;

use server::config::ZeusConfig;
use util::error::Result;

#[derive(Debug)]
pub enum ErrorKind {
}

pub trait Runnable: Send + 'static {
    fn run(&mut self);
}

pub trait ExecutorService: Sync + Send {
    fn submit(&self, task: Box<Runnable>) -> Result<()>;
    fn shutdown(&self) -> Result<usize>;
}

pub fn build(name: &str, config: &ZeusConfig) -> Result<Arc<ExecutorService>> {
    unimplemented!()
}
