mod cpupool_scheduler;

use std::sync::Arc;
use std::boxed::FnBox;


use server::config::ZeusConfig;
use self::cpupool_scheduler::CpuPoolScheduler;
use util::errors::*;

#[derive(Debug)]
pub enum ErrorKind {
}

pub type Callable = Box<FnBox() -> () + Send + 'static>;

pub struct Task {
  body: Callable,
}

impl Task {
  pub fn new(body: Callable) -> Task {
    Task {
      body,
    }
  }

  pub fn run(self) {
    (self.body)()
  }
}

pub trait ExecutorService: Sync + Send {
  fn submit(
    &self,
    task: Task,
  ) -> Result<()>;
  fn shutdown(&self) -> Result<usize>;
}

pub fn build(
  name: &str,
  config: &ZeusConfig,
) -> Result<Arc<ExecutorService>>
{
  Ok(Arc::new(CpuPoolScheduler::new(name, config)))
}
