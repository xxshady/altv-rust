use crate::{
    base_objects::{virtual_entity, virtual_entity_group},
    resource::Resource,
    sdk,
    vector::IntoVector3,
    world_object::WorldObject,
    SomeResult, VoidResult,
};
use std::ptr::NonNull;

impl virtual_entity::VirtualEntity {
    pub fn new(
        group: virtual_entity_group::VirtualEntityGroupContainer,
        pos: impl IntoVector3,
        streaming_distance: u32,
    ) -> SomeResult<virtual_entity::VirtualEntityContainer> {
        let pos = pos.into_vector3();
        let ptr = unsafe {
            sdk::ICore::CreateVirtualEntity(
                group.raw_ptr()?,
                pos.x(),
                pos.y(),
                pos.z(),
                streaming_distance,
            )
        };
        Ok(virtual_entity::add_to_pool!(NonNull::new(ptr).unwrap()))
    }

    pub fn id(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVirtualEntity::GetID(self.raw_ptr()?) })
    }

    pub fn streaming_distance(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVirtualEntity::GetStreamingDistance(self.raw_ptr()?) })
    }

    pub fn group(&self) -> SomeResult<virtual_entity_group::VirtualEntityGroupContainer> {
        let group_ptr = unsafe { sdk::IVirtualEntity::GetGroup(self.raw_ptr()?) };
        let group_ptr = NonNull::new(group_ptr).unwrap();

        let Some(group) = Resource::with_base_objects_mut(|v, _| v.virtual_entity_group.get_by_ptr(group_ptr)) else {
            anyhow::bail!("VirtualEntityGroup not found in the pool (usually this should never happen)")
        };
        Ok(group)
    }

    pub fn destroy(&mut self) -> VoidResult {
        virtual_entity::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

impl WorldObject for virtual_entity::VirtualEntity {}
