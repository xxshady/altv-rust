use std::ptr::NonNull;

use crate::{
    base_object::{
        extra_pools::{get_entity_by_id, wrappers::AnyEntity, Entity, EntityId},
        vehicle,
    },
    helpers::{self, IntoModelHash},
    resource::Resource,
    sdk,
    vector::IntoVector3,
    world_object::WorldObject,
    SomeResult, VoidResult,
};

impl vehicle::Vehicle {
    pub fn new(
        model: impl IntoModelHash,
        pos: impl IntoVector3,
        rot: impl IntoVector3,
    ) -> SomeResult<vehicle::VehicleContainer> {
        let pos = pos.into_vector3();
        let rot = rot.into_vector3();

        // TODO: check model before creation
        let ptr = Resource::with_pending_base_object_destroy_or_creation_mut(|_, _| unsafe {
            sdk::ICore::CreateVehicle(
                model.into_model_hash(),
                pos.x(),
                pos.y(),
                pos.z(),
                rot.x(),
                rot.y(),
                rot.z(),
            )
        });
        let Some(ptr) = NonNull::new(ptr) else {
            anyhow::bail!("Vehicle model is incorrect or there is no free id for new entity");
        };

        Ok(vehicle::add_to_pool!(ptr))
    }

    pub fn get_by_id(id: EntityId) -> SomeResult<vehicle::VehicleContainer> {
        get_entity_by_id!(AnyEntity::Vehicle, id).ok_or(anyhow::anyhow!("No vehicle with id: {id}"))
    }

    pub fn destroy(&mut self) -> VoidResult {
        vehicle::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

impl WorldObject for vehicle::Vehicle {}
impl Entity for vehicle::Vehicle {}
