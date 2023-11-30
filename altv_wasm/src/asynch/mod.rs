use futures::{
    executor::{LocalPool, LocalSpawner},
    task::{LocalSpawnExt, SpawnError},
    Future,
};

use crate::state::State;

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

pub fn spawn<Fut>(future: Fut) -> Result<(), SpawnError>
where
    Fut: Future<Output = ()> + 'static,
{
    State::with_async_executor_ref(|e, _| e.spawner.spawn_local(future))
}
