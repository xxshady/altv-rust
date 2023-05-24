use altv::prelude::*;
#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    // altv::events::on("js-send", |event| {
    //     let args: (bool, i32, [i32; 3], [i32; 4]) = event.args.deserialize()?;
    //     altv::log!("args: {args:?}");
    //     Ok(())
    // });

    // altv::events::emit("js-receive", &[&true, &123, &[1, 2, 3], &vec![1, 2, 3]])?;

    altv::events::on_player_connect(|c| {
        let kek = c.player.stream_synced_meta_entry("kek")?;
        kek.set(&"value".to_string())?;

        altv::events::emit(
            "js-eval",
            &[
                &"
            const [player] = args;
            alt.log('player.getStreamSyncedMeta(\"kek\"):', player.getStreamSyncedMeta('kek'));
            ",
                &[&c.player],
            ],
        )?;

        altv::events::emit_all_players("test", &[&"emit-all", &c.player])?;
        altv::events::emit_all_players_unreliable("test", &[&"emit-all-unreliable", &c.player])?;

        altv::events::emit_some_players("test", &[c.player.clone()], &[&"emit-some", &c.player])?;
        altv::events::emit_some_players_unreliable(
            "test",
            &[c.player.clone()],
            &[&"emit-some-unreliable", &c.player],
        )?;

        c.player.emit("test", &[&"emit-single", &c.player])?;
        c.player
            .emit_unreliable("test", &[&"emit-single-unreliable", &c.player])?;

        Ok(())
    });

    Ok(())
}
