use serde_wasm_bindgen::from_value;

use crate::{
  base_objects::{
    any_instances::AnyPlayer,
    base_object_js_ref::BaseObjectJsRef,
    attached_to_scope::AttachedToScope,
    handle::RawBaseObjectHandle,
    local_player::{local_player, LocalPlayerHandle},
    player::{PlayerHandle, ScopedPlayer},
    sdk_base_object_type::SdkBaseObjectType,
  },
  joaat::Joaat,
  wasm_imports::{self, is_local_player},
};

pub trait Entity: BaseObjectJsRef {
  fn model(&self) -> Joaat {
    self.js_ref().model()
  }
}

pub trait SyncedEntity<'scope>: BaseObjectJsRef + AttachedToScope<'scope> {
  fn net_owner(&'scope self) -> Option<AnyPlayer<'scope>> {
    let Some(player) = self.js_ref().net_owner() else {
      return None;
    };

    let net_owner = if is_local_player(&player) {
      AnyPlayer::Local(local_player())
    } else {
      let raw_handle = wasm_imports::get_base_object_raw_handle(&player);
      let raw_handle: RawBaseObjectHandle = from_value(raw_handle).unwrap();
      let player_handle = PlayerHandle::from_raw_handle(raw_handle);
      let scoped_player = player_handle.attach_to(self.attached_to_scope()).unwrap();
      AnyPlayer::Remote(scoped_player)
    };

    Some(net_owner)
  }
}
