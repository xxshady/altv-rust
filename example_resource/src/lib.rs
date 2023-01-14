use alt::specs::game_data::{GameData, StateData};
use alt::specs::{self, CoreApplication};
use alt::{resource_api::TestData, specs::state::State};
use std::sync::{Arc, Mutex};

fn test_interval() {
    println!("test_interval called");
}

pub struct GameState;

impl State for GameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        println!("State on_start");
    }

    fn tick(&mut self, data: StateData<GameData>) {
        println!("State tick");
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
// #[alt::res_main]
#[no_mangle]
pub fn main() -> CoreApplication {
    std::env::set_var("RUST_BACKTRACE", "1");

    // let resource = alt::MainResource::new(full_main_path, resource_api);

    // let callback = Arc::new(Mutex::new(alt::log as fn(&str)));

    // let test_data = Arc::new(Mutex::new(TestData {
    //     a: 10,
    //     callback: callback.clone(),
    // }));

    // alt::set_interval(test_interval, 1000, test_data.clone());

    // test_data.try_lock().unwrap().a += 1;
    // (callback.try_lock().unwrap())();

    let game_data_builder = specs::game_data::GameDataBuilder::new();
    let application = specs::ApplicationBuilder::new(Box::new(GameState)).build(game_data_builder);

    application
}
