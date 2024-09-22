use thiserror::Error;
use serde_wasm_bindgen::{from_value, to_value};

use crate::{
  base_objects::{any_instances::AnyPlayer, base_object_js_ref::BaseObjectJsRef},
  vector::Vector3,
};
use super::entity::SyncedEntity;

pub trait WorldObject: BaseObjectJsRef {
  fn pos(&self) -> Vector3 {
    // TODO: byte array for this stuff in a static variable for better perf?
    from_value(self.js_ref().pos()).unwrap()
  }

  fn dimension(&self) -> i32 {
    self.js_ref().dimension()
  }
}

pub trait LocalWorldObject: BaseObjectJsRef {
  fn set_pos(&self, value: &Vector3) {
    self.js_ref().set_pos(to_value(value).unwrap());
  }

  fn set_dimension(&self, value: i32) {
    self.js_ref().set_dimension(value);
  }
}

#[derive(Debug, Error)]
pub enum SetPosError {
  #[error("this world object is not owned by local player")]
  SetPosOnNotOwnedWorldObject,
}

pub trait SyncedWorldObject<'scope>: WorldObject + SyncedEntity<'scope> {
  fn set_pos(&'scope self, value: &Vector3) -> Result<(), SetPosError> {
    if matches!(self.net_owner(), Some(AnyPlayer::Local(_))) {
      self.js_ref().set_pos(to_value(value).unwrap());
      Ok(())
    } else {
      Err(SetPosError::SetPosOnNotOwnedWorldObject)
    }
  }
}
