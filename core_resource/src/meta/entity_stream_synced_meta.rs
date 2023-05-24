use std::{marker::PhantomData, rc::Rc};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    base_objects::{extra_pools::AnyEntity, ValidBaseObject},
    helpers::{self, IntoString},
    sdk, SomeResult,
};

pub struct StreamSyncedEntityMetaEntry<V> {
    pub(super) base_object: AnyEntity,
    pub(super) key: String,
    __type: PhantomData<V>,
}

pub trait StreamSyncedEntityMeta: ValidBaseObject
where
    AnyEntity: From<Rc<Self>>,
{
    /// Provides access to read or modify **stream synced** meta of Entity (Vehicle, Player, Ped, etc.).
    ///
    /// # Examples
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::meta::{BaseObjectMetaEntry, StreamSyncedEntityMeta};
    /// # fn test() -> altv::VoidResult {
    /// let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();
    ///
    /// let example_entry = vehicle.stream_synced_meta_entry("example")?;
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
        key: impl IntoString,
    ) -> SomeResult<StreamSyncedEntityMetaEntry<V>> {
        self.assert_valid()?;
        Ok(StreamSyncedEntityMetaEntry {
            base_object: self.clone().into(),
            key: key.into_string(),
            __type: PhantomData,
        })
    }

    fn stream_synced_meta_keys(self: &Rc<Self>) -> SomeResult<Vec<String>> {
        let entity: AnyEntity = self.clone().into();
        Ok(helpers::read_cpp_str_vec(unsafe {
            sdk::IEntity::GetStreamSyncedMetaDataKeys(entity.raw_ptr()?)
        }))
    }
}
