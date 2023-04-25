pub use altv::prelude::*;

#[altv::main]
fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    use altv::events;
    events::on_player_connect(|events::PlayerConnect { player }| {
        let name = player.name()?;
        altv::log!("player with name: {name} connected!");
        Ok(())
    });
}
