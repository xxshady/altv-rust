use serde_wasm_bindgen::to_value;

use crate::{base_objects::base_object_js_ref::BaseObjectJsRef, vector::Vector3};

pub trait LocalPlayer: BaseObjectJsRef {
  fn current_ammo(&self) -> u16 {
    self.js_ref().current_ammo()
  }

  // why not just implement SyncedWorldObject for LocalPlayer?
  // because pos setter cannot fail on local player
  fn set_pos(&self, value: &Vector3) {
    self.js_ref().set_pos(to_value(value).unwrap());
  }
}
