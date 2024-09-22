use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;

use crate::{timers::set_timeout, wasm_imports};

use super::{
  as_base_object_type::AsBaseObjectType,
  base_object_type::BaseObjectType,
  borrowed_instance::BorrowedBaseObject,
  class_traits::{
    self,
    entity::{Entity, SyncedEntity},
    world_object::WorldObject,
  },
  handle::{BaseObjectGeneration, BaseObjectHandle, BaseObjectId, GenericBaseObjectHandle},
  instance::BaseObject,
  manager::MANAGER_INSTANCE,
  scope::{new_scope, Scope},
  scoped_instance::ScopedBaseObject,
};

pub type Player = BaseObject<PlayerType>;
pub type ScopedPlayer<'scope> = ScopedBaseObject<'scope, PlayerType>;
pub type PlayerHandle = BaseObjectHandle<PlayerType>;

impl BorrowedBaseObject for PlayerHandle {}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct PlayerType;

impl AsBaseObjectType for PlayerType {
  fn as_base_object_type() -> BaseObjectType {
    BaseObjectType::Player
  }
}

impl Player {
  pub fn streamed_in<'scope>(scope: &'scope impl Scope) -> Vec<ScopedPlayer<'scope>> {
    let players = wasm_imports::get_streamed_in_players();
    let players: Vec<PlayerHandle> = from_value(players).unwrap();

    players
      .into_iter()
      .map(|handle| handle.attach_to(scope).unwrap_or_else(|| unreachable!()))
      .collect()
  }
}

impl<'scope> class_traits::player::Player for ScopedPlayer<'scope> {}

impl<'scope> WorldObject for ScopedPlayer<'scope> {}
impl<'scope> Entity for ScopedPlayer<'scope> {}
impl<'scope> SyncedEntity<'scope> for ScopedPlayer<'scope> {}

fn _test_player() {
  new_scope(|scope| {
    let players = Player::streamed_in(scope);
    let [player] = &players[..] else {
      return;
    };
    let player_handle = player.handle();

    set_timeout(
      move |scope| {
        let Some(_player) = player_handle.attach_to(scope) else {
          return;
        };
      },
      Duration::from_secs(1),
    );
  });
}
