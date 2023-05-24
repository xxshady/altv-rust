use crate::{base_objects::virtual_entity_group, helpers, sdk, SomeResult};

/// # **`VirtualEntityGroup implementation`**
impl virtual_entity_group::VirtualEntityGroup {
    pub fn new(max_entities_in_stream: u32) -> virtual_entity_group::VirtualEntityGroupContainer {
        helpers::create_base_object!(
            virtual_entity_group,
            sdk::ICore::CreateVirtualEntityGroup(max_entities_in_stream),
            panic!("Failed to create VirtualEntityGroup")
        )
    }

    pub fn max_entities_in_stream(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVirtualEntityGroup::GetMaxEntitiesInStream(self.raw_ptr()?) })
    }

    // cannot be destroyed
    // pub fn destroy(&self) -> VoidResult {
    //     virtual_entity_group::remove_from_pool!(self)?;
    //     self.internal_destroy()
    // }
}
