use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use std::{sync::OnceLock, time::Duration};

use crate::{timers::set_interval, vector::Vector3, wasm_imports};

use super::{
  as_base_object_type::AsBaseObjectType,
  attached_to_scope::AttachedToScope,
  base_object_type::BaseObjectType,
  class_traits::{
    self,
    entity::{Entity, SyncedEntity},
    player::Player,
    world_object::{SyncedWorldObject, WorldObject},
  },
  handle::BaseObjectHandle,
  instance::BaseObject,
  manager::MANAGER_INSTANCE,
  scope::Scope,
};

pub type LocalPlayer = BaseObject<LocalPlayerType>;
pub type LocalPlayerHandle = BaseObjectHandle<LocalPlayerType>;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct LocalPlayerType;

impl AsBaseObjectType for LocalPlayerType {
  fn as_base_object_type() -> BaseObjectType {
    BaseObjectType::LocalPlayer
  }
}

impl class_traits::local_player::LocalPlayer for LocalPlayer {}
impl Player for LocalPlayer {}
impl WorldObject for LocalPlayer {}
impl Entity for LocalPlayer {}
impl SyncedEntity<'static> for LocalPlayer {}

impl AttachedToScope<'static> for LocalPlayer {
  fn attached_to_scope(&'static self) -> &'static dyn Scope {
    local_player_scope()
  }
}

// doesn't matter here anyway because we don't have multi threading in WASM (yet?)
unsafe impl Send for LocalPlayer {}
unsafe impl Sync for LocalPlayer {}

pub fn local_player() -> &'static LocalPlayer {
  static INSTANCE: OnceLock<LocalPlayer> = OnceLock::new();

  INSTANCE.get_or_init(|| {
    // hard coded because local player is created from the start and will never be destroyed (it has *static* lifetime)
    let generation = 1;

    let js_ref = wasm_imports::get_local_player();
    let handle = LocalPlayerHandle::new(js_ref.id(), generation);
    let instance = LocalPlayer::new(handle, js_ref);

    MANAGER_INSTANCE.with_borrow_mut(|manager| {
      manager.on_create(handle.as_generic());
    });

    instance
  })
}

pub(crate) fn local_player_scope() -> &'static dyn Scope {
  struct LocalPlayerScope;
  impl Scope for LocalPlayerScope {}

  static INSTANCE: OnceLock<LocalPlayerScope> = OnceLock::new();
  INSTANCE.get_or_init(|| LocalPlayerScope)
}

#[wasm_bindgen]
pub fn test_local_player() {
  set_interval(
    |_| {
      use class_traits::local_player::LocalPlayer as _;

      let p = local_player();
      crate::logging::dbg!(p.dimension(), p.name(), p.net_owner());

      let pos = p.pos();

      p.set_pos(&Vector3 {
        x: pos.x,
        y: pos.y,
        z: pos.z + 1.0,
      });

      Ok(())
    },
    Duration::from_secs(1),
  );
}
