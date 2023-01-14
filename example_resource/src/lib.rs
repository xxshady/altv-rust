use alt::specs::game_data::{GameData, StateData};
use alt::specs::state::State;
use alt::specs::{self, CoreApplication};

pub struct GameState;

impl State for GameState {
    fn on_start(&mut self, _: StateData<GameData>) {
        println!("State on_start");
    }

    fn tick(&mut self, _: StateData<GameData>) {
        println!("State tick");
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub fn main() -> Result<CoreApplication, Box<dyn std::error::Error>> {
    std::env::set_var("RUST_BACKTRACE", "1");

    let game_data_builder = specs::game_data::GameDataBuilder::new();
    let application = specs::ApplicationBuilder::new(Box::new(GameState)).build(game_data_builder);

    Ok(application)
}
