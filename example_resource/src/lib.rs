pub use altv::prelude::*;

#[altv::main(crate_name = "altv")]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let channel = altv::VoiceChannel::new_spatial(10.0);
    let channel = channel.unwrap();

    {
        let channel = channel.clone();
        altv::events::on_player_connect(move |c| {
            altv::log!("player connect {}", c.player.name()?);
            dbg!(channel.players()?);
            channel.add_player(c.player.clone())?;
            dbg!(channel.players()?);
            Ok(())
        });
    }

    altv::events::on_player_disconnect(move |c| {
        altv::log!("player disconnect {}", c.player.name()?);
        dbg!(channel.players()?);

        let channel = channel.clone();
        altv::set_timeout(
            move || {
                dbg!(channel.players()?);
                Ok(())
            },
            500,
        );

        Ok(())
    });
}
