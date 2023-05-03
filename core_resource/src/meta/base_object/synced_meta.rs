use std::rc::Rc;

use crate::{
    base_objects::{BaseObjectContainer, BasePtr, ValidBaseObject},
    helpers::{self, IntoString},
    sdk, SomeResult,
};

pub struct SyncedBaseObjectMetaEntry<T, InheritPtrs: Clone> {
    pub(super) base_object: BaseObjectContainer<T, InheritPtrs>,
    pub(super) key: String,
}

pub trait SyncedBaseObjectMeta<T, InheritPtrs: Clone>: ValidBaseObject
where
    BaseObjectContainer<T, InheritPtrs>: From<Rc<Self>>,
{
    /// Provides access to read or modify synced meta of BaseObject (Vehicle, ColShape, Player, etc.).
    ///
    /// # Examples
    /// ```rust
    /// let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();
    ///
    /// // Set "example" key of synced meta to `123`
    /// vehicle.synced_meta_entry("example")?.set(123)?;
    ///
    /// // Read "example" key of synced meta
    /// vehicle.synced_meta_entry("example")?.get()?; // Some(MValue::I64(123))
    /// ```
    fn synced_meta_entry(
        self: &Rc<Self>,
        key: impl IntoString,
    ) -> SomeResult<SyncedBaseObjectMetaEntry<T, InheritPtrs>> {
        self.assert_valid()?;
        Ok(SyncedBaseObjectMetaEntry {
            base_object: self.clone().into(),
            key: key.into_string(),
        })
    }

    fn synced_meta_keys(self: &Rc<Self>) -> SomeResult<Vec<String>> {
        let base_object: BaseObjectContainer<T, InheritPtrs> = self.clone().into();
        Ok(helpers::read_cpp_str_vec(unsafe {
            sdk::IBaseObject::GetSyncedMetaDataKeys(base_object.raw_base_ptr()?)
        }))
    }
}
