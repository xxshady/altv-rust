use std::rc::Rc;

use crate::{
    base_objects::{virtual_entity::VirtualEntityContainer, ValidBaseObject},
    helpers::{self, IntoString},
    sdk, SomeResult,
};

pub struct StreamSyncedVirtualEntityMetaEntry {
    pub(super) base_object: VirtualEntityContainer,
    pub(super) key: String,
}

pub trait StreamSyncedVirtualEntityMeta: ValidBaseObject
where
    VirtualEntityContainer: From<Rc<Self>>,
{
    /// Provides access to read or modify stream synced meta of VirtualEntity.
    ///
    /// # Examples
    /// ```rust
    /// let group = altv::VirtualEntityGroup::new(10);
    /// let entity = altv::VirtualEntity::new(
    ///     group.clone(),
    ///     altv::Vector3::new(0, 0, 72),
    ///     10,
    /// )?;
    ///
    /// // Set "example" key of stream synced meta to `123`
    /// entity.stream_synced_meta_entry("example")?.set(123)?;
    ///
    /// // Read "example" key of stream synced meta
    /// entity.stream_synced_meta_entry("example")?.get()?; // Some(MValue::I64(123))
    /// ```
    fn stream_synced_meta_entry(
        self: &Rc<Self>,
        key: impl IntoString,
    ) -> SomeResult<StreamSyncedVirtualEntityMetaEntry> {
        self.assert_valid()?;
        Ok(StreamSyncedVirtualEntityMetaEntry {
            base_object: self.clone().into(),
            key: key.into_string(),
        })
    }

    fn stream_synced_meta_keys(self: &Rc<Self>) -> SomeResult<Vec<String>> {
        let entity: VirtualEntityContainer = self.clone().into();
        Ok(helpers::read_cpp_str_vec(unsafe {
            sdk::IVirtualEntity::GetStreamSyncedMetaDataKeys(entity.raw_ptr()?)
        }))
    }
}
