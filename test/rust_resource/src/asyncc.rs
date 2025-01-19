// Make sure to install "futures" crate
use futures::{
  executor::{LocalPool, LocalSpawner},
  task::LocalSpawnExt,
  task::SpawnError,
};
use std::{
  cell::RefCell,
  future::{poll_fn, Future},
  task::Poll,
  time::Duration,
};

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

pub fn spawn<F>(future: F) -> Result<(), SpawnError>
where
  F: Future<Output = ()> + 'static,
{
  EXECUTOR_INSTANCE.with_borrow(|executor| executor.spawner.spawn_local(future))
}

// pub fn block_on<F>(future: F)
// where
//   F: Future<Output = ()>,
// {
//   EXECUTOR_INSTANCE.with_borrow_mut(|executor| {
//     executor.pool.run_until(future);
//   });
// }

pub fn on_every_tick() {
  EXECUTOR_INSTANCE.with_borrow_mut(|executor| {
    executor.run();
  });
}

pub fn wait(duration: Duration) -> impl Future {
  let now = std::time::Instant::now();
  let mut timer_was_set = false;

  poll_fn(move |cx| {
    if timer_was_set {
      if now.elapsed() >= duration {
        return Poll::Ready(());
      }

      return Poll::Pending;
    }
    timer_was_set = true;

    let mut waker = Some(cx.waker().clone());
    altv::set_timeout(
      move || {
        waker.take().unwrap().wake();
      },
      duration.as_millis() as u64,
    );

    Poll::Pending
  })
}
