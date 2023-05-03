use std::rc::Rc;

use crate::{
    sdk,
    base_objects::{extra_pools::AnyEntity, ValidBaseObject},
    helpers::{self, IntoString},
    SomeResult,
};

pub struct StreamSyncedEntityMetaEntry {
    pub(super) base_object: AnyEntity,
    pub(super) key: String,
}

pub trait StreamSyncedEntityMeta: ValidBaseObject
where
    AnyEntity: From<Rc<Self>>,
{
    /// Provides access to read or modify stream synced meta of Vehicle.
    ///
    /// # Examples
    /// ```rust
    /// let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();
    ///
    /// // Set "example" key of stream synced meta to `123`
    /// vehicle.stream_synced_meta_entry("example")?.set(123)?;
    ///
    /// // Read "example" key of stream synced meta
    /// vehicle.stream_synced_meta_entry("example")?.get()?; // Some(MValue::I64(123))
    /// ```
    fn stream_synced_meta_entry(
        self: &Rc<Self>,
        key: impl IntoString,
    ) -> SomeResult<StreamSyncedEntityMetaEntry> {
        self.assert_valid()?;
        Ok(StreamSyncedEntityMetaEntry {
            base_object: self.clone().into(),
            key: key.into_string(),
        })
    }

    fn stream_synced_meta_keys(self: &Rc<Self>) -> SomeResult<Vec<String>> {
        let entity: AnyEntity = self.clone().into();
        Ok(helpers::read_cpp_str_vec(unsafe {
            sdk::IEntity::GetStreamSyncedMetaDataKeys(entity.raw_ptr()?)
        }))
    }
}
