use altv::prelude::*;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
  altv::log!("~gl~hello world");

  altv::events::on_player_connect(|ev| {
    on_player_connect(&ev.player)?;
    Ok(())
  });

  // After reloading of the resource all previously connected players
  // remain on the server so we need get them manually:
  for p in altv::Player::all() {
    on_player_connect(&p)?;
  }

  // This vehicle will be destroyed automatically at resource unloading
  let _vehicle = altv::Vehicle::new("sultan2", (3, 3, 75), 0)?;

  // TODO: metadata cleanup when resource is unloaded
  // (not implemented yet)

  Ok(())
}

// Called when resource is unloaded, for example
// by using `stop <resource name>` command in the server console
#[altv::before_unload]
fn before_unload() {
  altv::log!("before unload");
}

fn on_player_connect(player: &altv::PlayerContainer) -> altv::VoidResult {
  player.spawn("mp_m_freemode_01", (0, 0, 71))?;
  Ok(())
}
