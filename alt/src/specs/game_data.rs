use specs::{Dispatcher, DispatcherBuilder, RunNow, System, World};

pub trait DataInit<T> {
    fn build(self, world: &mut World) -> T;
}

pub trait DataDispose {
    fn dispose(&mut self, world: &mut World);
}

impl DataInit<()> for () {
    fn build(self, _world: &mut World) {}
}

impl DataDispose for () {
    fn dispose(&mut self, _world: &mut World) {}
}

impl DataDispose for GameData<'_, '_> {
    fn dispose(&mut self, world: &mut World) {
        self.dispose(world)
    }
}

pub struct StateData<'a, T> {
    pub world: &'a mut World,
    pub data: &'a mut T,
}

impl<'a, T> StateData<'a, T> {
    pub fn new(world: &'a mut World, data: &'a mut T) -> Self {
        StateData { world, data }
    }
}

pub struct GameData<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> GameData<'a, 'b> {
    pub fn new(dispatcher: Dispatcher<'a, 'b>) -> Self {
        GameData {
            dispatcher: Some(dispatcher),
        }
    }

    pub fn update(&mut self, world: &World) {
        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch_thread_local(world);
        }
    }

    pub fn dispose(&mut self, world: &mut World) {
        if let Some(dispatcher) = self.dispatcher.take() {
            dispatcher.dispose(world);
        }
    }
}

pub struct GameDataBuilder<'a, 'b> {
    dispatcher_builder: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> GameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        GameDataBuilder {
            dispatcher_builder: DispatcherBuilder::new(),
        }
    }

    pub fn with<T>(mut self, system: T, name: &str, dep: &[&str]) -> Self
    where
        T: for<'c> System<'c> + Send + 'a,
    {
        self.dispatcher_builder = self.dispatcher_builder.with(system, name, dep);
        self
    }

    pub fn with_thread_local<T>(mut self, system: T) -> Self
    where
        T: for<'c> RunNow<'c> + 'b,
    {
        self.dispatcher_builder = self.dispatcher_builder.with_thread_local(system);
        self
    }
}

impl<'a, 'b> DataInit<GameData<'a, 'b>> for GameDataBuilder<'a, 'b> {
    fn build(self, mut world: &mut World) -> GameData<'a, 'b> {
        let mut dispatcher = self.dispatcher_builder.build();
        dispatcher.setup(&mut world);

        GameData::new(dispatcher)
    }
}
