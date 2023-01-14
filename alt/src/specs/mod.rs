use specs::{Component, Dispatcher, DispatcherBuilder, RunNow, System, World, WorldExt};

pub mod game_data;
pub mod state;

use crate::specs::game_data::{DataInit, GameData, StateData};
use crate::specs::state::State;

pub struct ApplicationBuilder {
    world: World,
    state: Box<dyn State>,
}

impl ApplicationBuilder {
    pub fn new(state: Box<dyn State>) -> Self {
        let world = World::new();
        ApplicationBuilder { world, state }
    }

    pub fn register<C>(mut self) -> Self
    where
        C: Component,
        C::Storage: Default,
    {
        self.world.register::<C>();
        self
    }

    pub fn build<I>(mut self, init: I) -> CoreApplication
    where
        I: DataInit<GameData<'static, 'static>>,
    {
        let data = init.build(&mut self.world);

        CoreApplication {
            world: self.world,
            state: self.state,
            data,
        }
    }
}

pub struct CoreApplication {
    world: World,
    state: Box<dyn State>,
    data: GameData<'static, 'static>,
}

impl CoreApplication {
    pub fn start(&mut self) {
        self.state
            .on_start(StateData::new(&mut self.world, &mut self.data))
    }

    pub fn stop(&mut self) {
        self.state
            .on_stop(StateData::new(&mut self.world, &mut self.data))
    }

    pub fn tick(&mut self) {
        println!("CoreApplication tick, calling state.tick");
        self.state
            .tick(StateData::new(&mut self.world, &mut self.data));
        println!("called?");
        self.world.maintain();
        self.data.update(&mut self.world);
    }
}
