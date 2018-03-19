use futures::future::ok;
use futures_cpupool::CpuPool;
use futures_cpupool::Builder as CpuPoolBuilder;

use super::ExecutorService;
use super::Task;
use server::config::ZeusConfig;
use util::error::Result;

pub struct CpuPoolScheduler {
  cpu_pool: CpuPool,
}

impl CpuPoolScheduler {
  pub fn new(
    name: &str,
    config: &ZeusConfig,
  ) -> CpuPoolScheduler
  {
    let cpu_pool =
      CpuPoolBuilder::new().pool_size(config.query_config.worker_size).name_prefix(name).create();

    CpuPoolScheduler {
      cpu_pool,
    }
  }
}

impl ExecutorService for CpuPoolScheduler {
  fn submit(
    &self,
    task: Task,
  ) -> Result<()>
  {
    info!("Submitted a task to query scheduler!");
    self
      .cpu_pool
      .spawn_fn(move || {
        task.run();
        ok::<(), ()>(())
      })
      .forget();
    Ok(())
  }

  fn shutdown(&self) -> Result<usize> {
    info!("Query scheduler is closed!");
    Ok(0)
  }
}


#[cfg(test)]
mod tests {
  use futures::Future;
  use futures::sync::oneshot::Sender;
  use futures::sync::oneshot::channel;

  use scheduler::Task;
  use server::config::ZeusConfig;
  use scheduler::ExecutorService;
  use scheduler::cpupool_scheduler::CpuPoolScheduler;

  struct BarrierCount {
    pub num: u32,
    pub sink: Sender<u32>,
  }

  impl BarrierCount {
    fn run(self) {
      self.sink.send(self.num).ok().unwrap();
    }
  }

  #[test]
  fn test_sumbit_task() {
    let mut config = ZeusConfig::default();
    config.query_config.worker_size = 4;

    let scheduler = CpuPoolScheduler::new("test-runner", &config);

    let (sender, receiver) = channel();

    let counter = BarrierCount {
      num: 5,
      sink: sender,
    };

    let runner = Box::new(move || counter.run());

    scheduler.submit(Task::new(runner)).ok().unwrap();

    let r = receiver.wait().unwrap();

    assert_eq!(5, r);
  }
}
