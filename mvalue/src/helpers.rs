use altv_sdk::MValueType;

use crate::{to_mvalue, types::RawMutMValue, DynMValueArgs, Result};

#[macro_export]
macro_rules! __serialize_simple {
    ($self:ident, $create_mvalue:expr) => {{
        use autocxx::prelude::*;
        $self.output = Some($crate::wrappers::MutMValue::new(
            unsafe { $create_mvalue }.within_unique_ptr(),
        ));
        Ok(())
    }};
}

pub use __serialize_simple as serialize_simple;

#[macro_export]
macro_rules! __deserialize_simple {
    ($self:ident, $visitor:expr, $sdk_type:ident: $rust_type:ident) => {{
        paste::paste! {
            $self.assert_mvalue_type($self.mvalue_type()?, altv_sdk::MValueType::$sdk_type)?;
            $visitor.[<visit_ $rust_type>](unsafe { altv_sdk::ffi::[<read_mvalue_ $sdk_type:snake>]($self.input.get()) })
        }
    }};
}

pub use __deserialize_simple as deserialize_simple;

#[macro_export]
macro_rules! __generate_serde_via_bytes_for {
    (
        $value_type:ty,
        $expecting_value:literal,
        $serialization_key:path,
        $module_name:ident,
        $serialize_fields:expr,
        $deserialize_byte_buf:expr
    ) => {
        mod $module_name {
            use super::*;

            impl serde::Serialize for $value_type {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    let bytes = $crate::bytes_num::to_byte_buf($serialize_fields(self));
                    serializer.serialize_newtype_struct(
                        $serialization_key,
                        serde_bytes::Bytes::new(&bytes),
                    )
                }
            }

            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = $value_type;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(formatter, $expecting_value)
                }

                fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    $deserialize_byte_buf(v)
                }
            }

            impl<'de> serde::Deserialize<'de> for $value_type {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    deserializer.deserialize_newtype_struct($serialization_key, Visitor)
                }
            }
        }
    };
}

pub use __generate_serde_via_bytes_for as generate_serde_via_bytes_for;

pub(crate) fn sdk_type_to_rust(sdk_type: MValueType) -> &'static str {
    match sdk_type {
        MValueType::BaseObject => "BaseObject (Player, Vehicle, ColShape, etc.)", // for example altv::VehicleContainer
        MValueType::Bool => "bool",
        MValueType::ByteArray => "ByteArray (ByteBuf)", // altv::ByteBuf
        MValueType::Dict => "Dict (HashMap or struct)",
        MValueType::Double => "Double (f64 or f32)",
        MValueType::Function => "Function", // not implemented (yet?)
        MValueType::Int => "Int (i64..u8)",
        MValueType::Uint => "UInt (u64..u8)",
        MValueType::List => "List (tuple or static array or Vec or slice)",
        MValueType::Nil | MValueType::None => "None",
        MValueType::Rgba => "Rgba", // altv::RGBA
        MValueType::String => "String",
        MValueType::Vector2 => "Vector2", // altv::Vector2
        MValueType::Vector3 => "Vector3", // altv::Vector3
    }
}

pub fn serialize_args(args: DynMValueArgs) -> Result<RawMutMValue> {
    Ok(to_mvalue(args)?.into_raw())
}
