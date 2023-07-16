use std::{marker::PhantomData, rc::Rc};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    base_objects::{virtual_entity::VirtualEntityRc, ValidBaseObject},
    helpers, sdk, SomeResult,
};

pub struct StreamSyncedVirtualEntityMetaEntry<V> {
    pub(super) base_object: VirtualEntityRc,
    pub(super) key: String,
    __type: PhantomData<V>,
}

pub trait StreamSyncedVirtualEntityMeta: ValidBaseObject
where
    VirtualEntityRc: From<Rc<Self>>,
{
    /// Provides access to read or modify **stream synced** meta of VirtualEntity.
    ///
    /// # Examples
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::meta::{BaseObjectMetaEntry, StreamSyncedVirtualEntityMeta};
    /// # fn test() -> altv::VoidResult {
    /// let group = altv::VirtualEntityGroup::new(10);
    /// let virtual_entity = altv::VirtualEntity::new(
    ///     group.clone(),
    ///     altv::Vector3::new(0, 0, 72),
    ///     10,
    /// )?;
    ///
    /// let example_entry = virtual_entity.stream_synced_meta_entry("example")?;
    ///
    /// // Set "example" key of stream synced meta to `123`
    /// example_entry.set(&123)?;
    ///
    /// // Read "example" key of stream synced meta
    /// let value: Option<i32> = example_entry.get()?; // Some(123)
    /// # Ok(()) }
    /// ```
    fn stream_synced_meta_entry<V: Serialize + DeserializeOwned>(
        self: &Rc<Self>,
        key: impl ToString,
    ) -> SomeResult<StreamSyncedVirtualEntityMetaEntry<V>> {
        self.assert_valid()?;
        let virtual_entity: VirtualEntityRc = self.clone().into();
        Ok(StreamSyncedVirtualEntityMetaEntry {
            base_object: virtual_entity,
            key: key.to_string(),
            __type: PhantomData,
        })
    }

    fn stream_synced_meta_keys(self: &Rc<Self>) -> SomeResult<Vec<String>> {
        let virtual_entity: VirtualEntityRc = self.clone().into();
        Ok(helpers::read_cpp_str_vec(unsafe {
            sdk::IVirtualEntity::GetStreamSyncedMetaDataKeys(virtual_entity.raw_ptr()?)
        }))
    }
}
