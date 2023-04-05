use crate::{
    base_objects::{virtual_entity, virtual_entity_group},
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
                group.try_borrow()?.ptr()?.as_ptr(),
                pos.x(),
                pos.y(),
                pos.z(),
                streaming_distance,
            )
        };
        Ok(virtual_entity::add_to_pool!(NonNull::new(ptr).unwrap()))
    }

    pub fn destroy(&mut self) -> VoidResult {
        virtual_entity::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

impl WorldObject for virtual_entity::VirtualEntity {}
