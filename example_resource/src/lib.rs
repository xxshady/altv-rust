pub use altv::prelude::*;

#[altv::main]
fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::events::on_player_connect(|c| {
        let player = c.player.clone();

        altv::log!("player connected: {}", player.name()?);

        altv::set_interval(
            move || {
                let ped = altv::Ped::new(player.model()?, player.pos()?, player.rot()?)?;
                altv::log!("spawned ped id: {}", ped.id()?);
                altv::log!("spawned ped pos: {:?}", ped.pos()?);
                altv::log!("player pos: {:?}", player.pos()?);
                altv::log!("ped current weapon: {}", ped.current_weapon()?);
                altv::log!("player current weapon: {}", player.current_weapon()?);

                ped.set_health(1000)?;

                Ok(())
            },
            1000,
        );

        altv::set_interval(
            || {
                altv::log!("~rl~destroying all peds: {}", altv::Ped::all().len());
                altv::Ped::all().iter().for_each(|p| p.destroy().unwrap());
            },
            5000,
        );

        Ok(())
    });
}
