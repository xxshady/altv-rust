use futures::{
    executor::{LocalPool, LocalSpawner},
    task::{LocalSpawnExt, SpawnError},
    Future,
};

use crate::{state::State, IntoVoidResult};

pub(crate) mod timer;

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

pub fn spawn<R, F>(future: F) -> Result<(), SpawnError>
where
    R: IntoVoidResult,
    F: Future<Output = R> + 'static,
{
    State::with_async_executor_ref(|e, _| {
        e.spawner.spawn_local(async {
            let res = future.await.into_void_result();
            if let Err(err) = res {
                logger::error!("Spawned task failed with error: {err:?}");
            }
        })
    })
}
