#[alt::main(crate_name = "alt")]
fn main() -> impl alt::IntoVoidResult {
  // std::env::set_var("RUST_BACKTRACE", "1");
  alt::log!("~gl~hello world");

  // alt::log!("bt:\n{}", std::backtrace::Backtrace::capture());

  use alt::prelude::*;

  alt::events::on_player("test", |ev| {
    let args: (alt::mvalue::AnyMValue, alt::mvalue::AnyMValue) = ev.args.deserialize()?;
    dbg!(args);
    Ok(())
  });

  alt::events::on_player_connect(|ev| {
    ev.player.spawn("mp_m_freemode_01", (0, 0, 71))?;
    ev.player.emit("test", &[&123, &"test"])?;
    Ok(())
  });

  alt::events::on_console_command(|ev| {
    match ev.name.as_str() {
      "d" => {
        dbg!();
        let players = alt::Player::all();
        let Some(player) = players.get(0) else {
          dbg!();
          return Ok(());
        };

        player.emit("test", &[&125, &"test"])?;
      }
      "w" => {
        dbg!();
        std::mem::forget(vec![1_u8; 1024 * 1024 * 10]);
      }
      _ => {}
    }

    Ok(())
  });

  // alt::anyhow::bail!("this is an error");
}

#[alt::before_unload]
fn before_unload() {
  alt::log_error!("before unload");
}
