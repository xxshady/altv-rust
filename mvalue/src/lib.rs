mod de;
mod error;
mod ser;

pub mod bytes_num;
mod de_dict_key;
mod de_mvalue_slice;
mod helpers;
mod ser_base_object;
mod ser_dict_key;
pub mod ser_rgba;
pub mod ser_vector2;
pub mod ser_vector3;
mod types;
mod wrappers;

pub use de::{from_mvalue, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_mvalue, Serializer};

pub use ser_base_object::BASE_OBJECT_MVALUE;

pub use helpers::generate_serde_via_bytes_for;

pub use wrappers::ConstMValue;
pub use wrappers::MutMValue;

pub type DynMValue<'a> = &'a dyn erased_serde::Serialize;
pub type DynMValueArgs<'a> = &'a [DynMValue<'a>];

pub use de_mvalue_slice::{from_mvalue_slice, DeserializeMValueArgs};
