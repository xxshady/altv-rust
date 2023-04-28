use autocxx::WithinUniquePtr;

use crate::{
    base_objects::{inherit_ptrs, meta, virtual_entity, virtual_entity_group},
    helpers::IntoString,
    mvalue,
    resource::Resource,
    sdk,
    vector::Vector3,
    world_object::WorldObject,
    SomeResult, VoidResult,
};
use std::{collections::HashMap, ptr::NonNull};

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
    /// let group = altv::VirtualEntityGroup::new(10);
    /// let entity = altv::VirtualEntity::new(
    ///     group.clone(),
    ///     altv::Vector3::new(0, 0, 72),
    ///     10,
    /// )
    /// .unwrap();
    /// ```
    pub fn new(
        group: virtual_entity_group::VirtualEntityGroupContainer,
        pos: impl Into<Vector3>,
        streaming_distance: u32,
    ) -> SomeResult<virtual_entity::VirtualEntityContainer> {
        Self::new_with_stream_meta(
            group,
            pos,
            streaming_distance,
            HashMap::<String, mvalue::MValueNone>::new(),
        )
    }

    /// Creates new instance of VirtualEntity with initial stream synced meta.
    ///
    /// # Errors
    /// When provided `group` instance is invalid (destroyed).
    ///
    /// # Examples
    /// ```rust
    /// let group = altv::VirtualEntityGroup::new(10);
    /// let entity = altv::VirtualEntity::new_with_stream_meta(
    ///     group.clone(),
    ///     altv::Vector3::new(0, 0, 72),
    ///     10,
    ///     HashMap::from([("example", 123)]),
    /// )
    /// .unwrap();
    /// ```
    pub fn new_with_stream_meta(
        group: virtual_entity_group::VirtualEntityGroupContainer,
        pos: impl Into<Vector3>,
        streaming_distance: u32,
        stream_synced_meta: HashMap<
            impl IntoString,
            impl TryInto<mvalue::Serializable, Error = anyhow::Error>,
        >,
    ) -> SomeResult<virtual_entity::VirtualEntityContainer> {
        let pos = pos.into();

        let mut mvalue_map = unsafe { sdk::create_mvalue_unordered_map() }.within_unique_ptr();
        for (key, value) in stream_synced_meta {
            unsafe {
                sdk::push_to_mvalue_unordered_map(
                    mvalue_map.as_mut().unwrap(),
                    key.into_string(),
                    value
                        .try_into()
                        .or_else(|e| {
                            anyhow::bail!("Failed to convert value into mvalue, error: {e:?}")
                        })?
                        .0,
                )
            }
        }

        let ptr = unsafe {
            sdk::ICore::CreateVirtualEntity(
                group.raw_ptr()?,
                pos.x(),
                pos.y(),
                pos.z(),
                streaming_distance,
                mvalue_map,
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

impl WorldObject<inherit_ptrs::WorldObject> for virtual_entity::VirtualEntity {}

meta::impl_meta_type_for!(
    StreamSyncedMeta,
    virtual_entity::VirtualEntity,
    sdk::IVirtualEntity,
    virtual_entity::VirtualEntity::raw_ptr
);
