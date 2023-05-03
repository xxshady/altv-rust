use std::rc::Rc;

use crate::{
    base_objects::{BaseObjectContainer, ValidBaseObject},
    helpers::IntoString,
    SomeResult,
};

pub struct NormalBaseObjectMetaEntry<T, InheritPtrs: Clone> {
    pub(super) base_object: BaseObjectContainer<T, InheritPtrs>,
    pub(super) key: String,
}

pub trait NormalBaseObjectMeta<T, InheritPtrs: Clone>: ValidBaseObject
where
    BaseObjectContainer<T, InheritPtrs>: From<Rc<Self>>,
{
    /// Provides access to read or modify normal meta of BaseObject (Vehicle, ColShape, Player, etc.).
    ///
    /// # Examples
    /// ```rust
    /// let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();
    ///
    /// // Set "example" key of normal meta to `123`
    /// vehicle.meta_entry("example")?.set(123)?;
    ///
    /// // Read "example" key of normal meta
    /// vehicle.meta_entry("example")?.get()?; // Some(MValue::I64(123))
    /// ```
    fn meta_entry(
        self: &Rc<Self>,
        key: impl IntoString,
    ) -> SomeResult<NormalBaseObjectMetaEntry<T, InheritPtrs>> {
        self.assert_valid()?;
        Ok(NormalBaseObjectMetaEntry {
            base_object: self.clone().into(),
            key: key.into_string(),
        })
    }
}