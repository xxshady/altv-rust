use std::ptr::NonNull;

use serde::{de, ser};

use super::{
    super::{vehicle, AnyBaseObject},
    base_ptr::BasePtr,
    BaseObjectContainer,
};
use crate::resource::Resource;

// see `serialize_u64` method in mvalue/src/ser_base_object.rs
impl<T, InheritPtrs: Clone> ser::Serialize for BaseObjectContainer<T, InheritPtrs> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        // here its defaults to null ptr because i don't know how to return custom error here.
        // if ptr is null, mvalue implementation will return InvalidBaseObject error
        let raw_ptr = self.raw_base_ptr().unwrap_or(std::ptr::null_mut());

        serializer.serialize_newtype_struct(mvalue::BASE_OBJECT_MVALUE, &(raw_ptr as u64))
    }
}

struct BaseObjectVisitor;

impl<'de> de::Visitor<'de> for BaseObjectVisitor {
    type Value = AnyBaseObject;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Base Object MValue")
    }

    fn visit_u64<E>(self, base_object_ptr: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let Some(ptr) = NonNull::new(base_object_ptr as altv_sdk::BaseObjectRawMutPtr) else {
            return Err(E::custom("Base object is invalid (it has probably already been destroyed)"));
        };

        let Some(base_object) = Resource::with_base_objects_ref(|v, _| v.get_by_ptr(ptr)) else {
            return Err(E::custom("Base object is not found in the pool (it has probably already been destroyed)"));
        };

        Ok(base_object)
    }
}

impl<'de> de::Deserialize<'de> for AnyBaseObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(BaseObjectVisitor)
    }
}

#[macro_export]
macro_rules! __impl_mvalue_deserialize_for_base_object {
    ($container:ty, $class_name:ident, $module_name:ident) => {
        mod $module_name {
            use serde::{de};
            use std::ptr::NonNull;
            use super::*;
            use $crate::resource::Resource;

            struct BaseObjectVisitor;

            impl<'de> de::Visitor<'de> for BaseObjectVisitor {
                type Value = $container;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(formatter, "{} MValue", stringify!($class_name))
                }

                fn visit_u64<E>(self, base_object_ptr: u64) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    let Some(ptr) = NonNull::new(base_object_ptr as altv_sdk::BaseObjectRawMutPtr) else {
                        return Err(E::custom(format!("{} is invalid (it has probably already been destroyed)", stringify!($class_name))));
                    };

                    let Some(base_object) = Resource::with_base_objects_ref(|v, _| v.get_by_ptr(ptr)) else {
                        return Err(E::custom(format!("{} is not found in the pool (it has probably already been destroyed)", stringify!($class_name))));
                    };

                    match base_object {
                        AnyBaseObject::$class_name(v) => Ok(v),
                        _ => Err(E::custom(format!("This base object is not {}", stringify!($class_name)))),
                    }
                }
            }

            impl<'de> de::Deserialize<'de> for $container {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: de::Deserializer<'de>,
                {
                    deserializer.deserialize_any(BaseObjectVisitor)
                }
            }
        }
    };
}

pub use __impl_mvalue_deserialize_for_base_object as impl_deserialize_for;
