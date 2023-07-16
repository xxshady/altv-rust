use std::{marker::PhantomData, rc::Rc};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    base_objects::{BaseObjectWrapperRc, BasePtr, ValidBaseObject},
    helpers, sdk, SomeResult,
};

pub struct NormalBaseObjectMetaEntry<V, T, InheritPtrs: Clone> {
    pub(super) base_object: BaseObjectWrapperRc<T, InheritPtrs>,
    pub(super) key: String,
    __type: PhantomData<V>,
}

pub trait NormalBaseObjectMeta<T, InheritPtrs: Clone>: ValidBaseObject
where
    BaseObjectWrapperRc<T, InheritPtrs>: From<Rc<Self>>,
    InheritPtrs: Clone,
{
    /// Provides access to read or modify normal meta of BaseObject (Vehicle, ColShape, Player, etc.).
    ///
    /// # Examples
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::meta::{BaseObjectMetaEntry, NormalBaseObjectMeta};
    /// # fn test() -> altv::VoidResult {
    /// let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();
    ///
    /// let example_entry = vehicle.meta_entry("example")?;
    ///
    /// // Set "example" key of normal meta to `123`
    /// example_entry.set(&123)?;
    ///
    /// // Read "example" key of normal meta
    /// let value: Option<i32> = example_entry.get()?; // Some(123)
    /// # Ok(()) }
    /// ```
    fn meta_entry<V: Serialize + DeserializeOwned>(
        self: &Rc<Self>,
        key: impl ToString,
    ) -> SomeResult<NormalBaseObjectMetaEntry<V, T, InheritPtrs>> {
        self.assert_valid()?;
        let base_object: BaseObjectWrapperRc<T, InheritPtrs> = self.clone().into();
        Ok(NormalBaseObjectMetaEntry {
            base_object,
            key: key.to_string(),
            __type: PhantomData,
        })
    }

    fn meta_keys(self: &Rc<Self>) -> SomeResult<Vec<String>> {
        let base_object: BaseObjectWrapperRc<T, InheritPtrs> = self.clone().into();
        Ok(helpers::read_cpp_str_vec(unsafe {
            sdk::IBaseObject::GetMetaDataKeys(base_object.raw_base_ptr()?)
        }))
    }
}
