use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use std::time::Duration;

use crate::{timers::set_interval, vector::Vector3, wasm_imports};

use super::{
  as_base_object_type::AsBaseObjectType,
  attached_to_scope::AttachedToScope,
  base_object_type::BaseObjectType,
  borrowed_instance::BorrowedBaseObject,
  class_traits::{
    self,
    entity::{Entity, SyncedEntity},
    world_object::{SyncedWorldObject, WorldObject},
  },
  handle::{BaseObjectGeneration, BaseObjectHandle, BaseObjectId, GenericBaseObjectHandle},
  instance::BaseObject,
  manager::MANAGER_INSTANCE,
  scope::Scope,
  scoped_instance::ScopedBaseObject,
};

pub type Vehicle = BaseObject<VehicleType>;
pub type ScopedVehicle<'scope> = ScopedBaseObject<'scope, VehicleType>;
pub type VehicleHandle = BaseObjectHandle<VehicleType>;

impl BorrowedBaseObject for VehicleHandle {}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct VehicleType;

impl AsBaseObjectType for VehicleType {
  fn as_base_object_type() -> BaseObjectType {
    BaseObjectType::Vehicle
  }
}

impl Vehicle {
  // TODO:
  pub fn streamed_in<'scope>(scope: &'scope impl Scope) -> Vec<ScopedVehicle<'scope>> {
    let vehicles = wasm_imports::get_streamed_in_vehicles();
    let vehicles: Vec<VehicleHandle> = from_value(vehicles).unwrap();

    vehicles
      .into_iter()
      .map(|handle| handle.attach_to(scope).unwrap_or_else(|| unreachable!()))
      .collect()
  }
}

impl<'scope> class_traits::vehicle::Vehicle for ScopedVehicle<'scope> {}
impl<'scope> WorldObject for ScopedVehicle<'scope> {}
impl<'scope> SyncedWorldObject<'scope> for ScopedVehicle<'scope> {}
impl<'scope> Entity for ScopedVehicle<'scope> {}
impl<'scope> SyncedEntity<'scope> for ScopedVehicle<'scope> {}

#[wasm_bindgen]
pub fn test_vehicle() {
  set_interval(
    |scope| {
      let vehicles = Vehicle::streamed_in(scope);
      crate::logging::dbg!(&vehicles);
      for v in vehicles {
        crate::logging::dbg!(v.net_owner());
        let pos = v.pos();

        // TODO: altv sets vehicle position for one frame? check if there any issue opened for it
        crate::logging::dbg!(&v.set_pos(&Vector3 {
          x: pos.x,
          y: pos.y,
          z: pos.z + 2.0,
        }));
      }
    },
    Duration::from_secs(1),
  );
}
