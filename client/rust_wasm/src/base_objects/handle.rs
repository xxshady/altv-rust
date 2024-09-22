use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;

use crate::wasm_imports;

use super::{
  as_base_object_type::AsBaseObjectType,
  base_object_type::{rust_to_sdk_base_object_type, sdk_to_rust_base_object_type, BaseObjectType},
  borrowed_instance::BorrowedBaseObject,
  instance::BaseObject,
  local_player::LocalPlayerHandle,
  manager::MANAGER_INSTANCE,
  player::PlayerHandle,
  scope::{self, Scope},
  scoped_instance::ScopedBaseObject,
  sdk_base_object_type::SdkBaseObjectType,
  vehicle::VehicleHandle,
};

pub type BaseObjectId = u32;
pub type BaseObjectGeneration = u64;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct GenericBaseObjectHandle {
  pub btype: BaseObjectType,
  pub id: BaseObjectId,
  pub generation: BaseObjectGeneration,
}

impl GenericBaseObjectHandle {
  pub fn js_ref(&self) -> wasm_imports::BaseObject {
    let (sdk_type, is_remote) = rust_to_sdk_base_object_type(self.btype);
    let Some(base_object_ref) =
      wasm_imports::get_base_object_ref(sdk_type as u8, is_remote, self.id)
    else {
      panic!("Expected valid base object: {self:?}");
    };
    base_object_ref
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RawBaseObjectHandle {
  pub sdk_type: SdkBaseObjectType,
  pub is_remote: bool,
  pub id: BaseObjectId,
  pub generation: BaseObjectGeneration,
}

impl RawBaseObjectHandle {
  pub fn as_handle(&self) -> GenericBaseObjectHandle {
    GenericBaseObjectHandle {
      btype: sdk_to_rust_base_object_type(self.sdk_type, self.is_remote),
      id: self.id,
      generation: self.generation,
    }
  }
}

/// Owned identifier of base object, not attached to any scope.
///
/// # Handle is unique identifier
/// It's guaranteed that when you try to attach it to some scope it won't give you wrong instance
/// (for example, in alt:V JS API when you destroy one base object and create another one alt:V reuses ID, thus causing this bug: https://github.com/altmp/altv-js-module/issues/289)
///
/// # Examples
///
/// ```
/// altv::events::on_game_entity_create(|one_scope| {
///   let altv::AnyEntity::Player(player) = one_scope.entity {
///     return;
///   };
///   
///   // Now you can do whatever you want with it:
///   // send it over network, store it in a static variable, etc.
///   let player_handle = player.handle();
///   
///   altv::set_timeout(|another_scope| {
///     let Some(player) = player_handle.attach_to(another_scope) else {
///       return;
///     };
///     dbg!(player);
///   }, Duration::from_secs(1));
/// });
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BaseObjectHandle<T: AsBaseObjectType> {
  id: BaseObjectId,
  generation: BaseObjectGeneration,
  #[serde(skip_serializing, skip_deserializing)]
  _t: PhantomData<T>,
}

impl<T: AsBaseObjectType> BaseObjectHandle<T> {
  pub(crate) fn new(id: BaseObjectId, generation: BaseObjectGeneration) -> Self {
    Self {
      id,
      generation,
      _t: PhantomData,
    }
  }

  pub(crate) fn from_raw_handle(raw_handle: RawBaseObjectHandle) -> Self {
    Self {
      id: raw_handle.id,
      generation: raw_handle.generation,
      _t: PhantomData,
    }
  }

  pub fn as_generic(self) -> GenericBaseObjectHandle {
    GenericBaseObjectHandle {
      btype: T::as_base_object_type(),
      id: self.id,
      generation: self.generation,
    }
  }

  /// Returns `None` if base object behind the handle has been destroyed.
  pub fn attach_to<'scope>(self, scope: &'scope dyn Scope) -> Option<ScopedBaseObject<'scope, T>>
  where
    Self: BorrowedBaseObject,
  {
    MANAGER_INSTANCE.with_borrow(|manager| {
      let player = BaseObject::new_by_handle(manager, self)?;
      Some(scope::attach_base_object(scope, player))
    })
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(u8)]
pub enum AnyBaseObjectHandle {
  Vehicle(VehicleHandle),
  Player(PlayerHandle),
  LocalPlayer(LocalPlayerHandle),
}
