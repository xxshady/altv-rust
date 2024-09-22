use futures::{
  executor::{LocalPool, LocalSpawner},
  task::LocalSpawnExt,
};
use std::{cell::RefCell, future::Future};

thread_local! {
  pub(crate) static EXECUTOR_INSTANCE: RefCell<Executor> = Default::default();
}

#[derive(Debug)]
pub(crate) struct Executor {
  pool: LocalPool,
  spawner: LocalSpawner,
}

impl Executor {
  pub(crate) fn run(&mut self) {
    self.pool.run_until_stalled();
  }
}

impl Default for Executor {
  fn default() -> Self {
    let pool = LocalPool::new();
    let spawner = pool.spawner();
    Self { pool, spawner }
  }
}

pub fn spawn_future<F>(future: F)
where
  F: Future<Output = ()> + 'static,
{
  EXECUTOR_INSTANCE
    .with_borrow(|executor| executor.spawner.spawn_local(future))
    .unwrap_or_else(|e| {
      panic!("Failed to spawn future, error: {e}");
    });
}
