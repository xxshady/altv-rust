use crate::{
    base_objects::virtual_entity_group, sdk, world_object::WorldObject, SomeResult, VoidResult,
};
use std::ptr::NonNull;

impl virtual_entity_group::VirtualEntityGroup {
    pub fn new(
        max_entities_in_stream: u32,
    ) -> SomeResult<virtual_entity_group::VirtualEntityGroupContainer> {
        let ptr = unsafe { sdk::ICore::CreateVirtualEntityGroup(max_entities_in_stream) };
        Ok(virtual_entity_group::add_to_pool!(
            NonNull::new(ptr).unwrap()
        ))
    }

    pub fn destroy(&mut self) -> VoidResult {
        virtual_entity_group::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

impl WorldObject for virtual_entity_group::VirtualEntityGroup {}
