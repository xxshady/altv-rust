use std::ptr::NonNull;

use crate::{
    base_objects::{virtual_entity, virtual_entity_group},
    helpers,
    meta::ve_stream_synced_meta::StreamSyncedVirtualEntityMeta,
    resource::Resource,
    sdk,
    vector::Vector3,
    SomeResult, VoidResult,
    exports::MValueHashMap,
};

/// # **`VirtualEntity implementation`**
impl virtual_entity::VirtualEntity {
    /// Creates new instance of VirtualEntity without initial stream synced meta.
    /// See [virtual_entity::VirtualEntity::new_with_stream_meta]
    /// if you want to create virtual entity with initial stream synced meta.
    ///
    /// # Errors
    /// When provided `group` instance is invalid (destroyed).
    ///
    /// # Examples
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # fn test() -> altv::VoidResult {
    /// let group = altv::VirtualEntityGroup::new(10);
    /// let entity = altv::VirtualEntity::new(
    ///     group.clone(),
    ///     altv::Vector3::new(0, 0, 72),
    ///     10,
    /// )?;
    /// # Ok(()) }
    /// ```
    pub fn new(
        group: virtual_entity_group::VirtualEntityGroupContainer,
        pos: impl Into<Vector3>,
        streaming_distance: u32,
    ) -> SomeResult<virtual_entity::VirtualEntityContainer> {
        Self::new_with_stream_meta(group, pos, streaming_distance, Default::default())
    }

    /// Creates new instance of VirtualEntity with initial stream synced meta.
    ///
    /// # Errors
    /// When provided `group` instance is invalid (destroyed).
    ///
    /// # Examples
    /// ```rust
    /// # mod altv {
    /// #     pub use altv_internal_core_resource::exports::*;
    /// #     pub use mvalue::DynMValue;
    /// # }
    /// # fn test() -> altv::VoidResult {
    /// # use std::collections::HashMap;
    /// let group = altv::VirtualEntityGroup::new(10);
    /// let entity = altv::VirtualEntity::new_with_stream_meta(
    ///     group.clone(),
    ///     altv::Vector3::new(0, 0, 72),
    ///     10,
    ///     altv::MValueHashMap::new(
    ///         HashMap::from([("example".to_string(), &123 as altv::DynMValue)])
    ///     ),
    /// )?;
    /// # Ok(()) }
    /// ```
    pub fn new_with_stream_meta(
        group: virtual_entity_group::VirtualEntityGroupContainer,
        pos: impl Into<Vector3>,
        streaming_distance: u32,
        stream_synced_meta: MValueHashMap,
    ) -> SomeResult<virtual_entity::VirtualEntityContainer> {
        let group = group.raw_ptr()?;
        let pos = pos.into();
        let meta = stream_synced_meta.to_cpp()?;

        Ok(helpers::create_base_object!(
            virtual_entity,
            sdk::ICore::CreateVirtualEntity(
                group,
                pos.x(),
                pos.y(),
                pos.z(),
                streaming_distance,
                meta,
            ),
            panic!("Failed to create VirtualEntity")
        ))
    }

    pub fn streaming_distance(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVirtualEntity::GetStreamingDistance(self.raw_ptr()?) })
    }

    pub fn group(&self) -> SomeResult<virtual_entity_group::VirtualEntityGroupContainer> {
        let group_ptr = unsafe { sdk::IVirtualEntity::GetGroup(self.raw_ptr()?) };
        let group_ptr = NonNull::new(group_ptr).unwrap();

        let Some(group) =
            Resource::with_base_objects_mut(|v, _| v.virtual_entity_group.get_by_ptr(group_ptr))
        else {
            anyhow::bail!(
                "VirtualEntityGroup not found in the pool (usually this should never happen)"
            )
        };
        Ok(group)
    }

    pub fn visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVirtualEntity::IsVisible(self.raw_ptr()?) })
    }

    pub fn set_visible(&self, toggle: bool) -> VoidResult {
        unsafe { sdk::IVirtualEntity::SetVisible(self.raw_ptr()?, toggle) }
        Ok(())
    }

    pub fn destroy(&self) -> VoidResult {
        virtual_entity::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

impl StreamSyncedVirtualEntityMeta for virtual_entity::VirtualEntity {}
