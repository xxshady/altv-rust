use crate::specs::game_data::{GameData, StateData};

pub trait State {
    fn on_start(&mut self, data: StateData<GameData>) {
        let _ = data;
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let _ = data;
    }

    fn tick(&mut self, data: StateData<GameData>) {
        let _ = data;
    }
}
