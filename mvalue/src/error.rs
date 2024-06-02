use std::fmt::Display;

use altv_sdk::MValueType;
use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    SerializationFailed,
    DictKeySerializationFailed,
    BaseObjectSerializationFailed,
    Vector3SerializationFailed,
    Vector3DeserializationFailed,
    Vector2SerializationFailed,
    Vector2DeserializationFailed,
    RgbaSerializationFailed,
    RgbaDeserializationFailed,
    InvalidBaseObject,
    InvalidMValueType,
    DictKeyMustBeAString,
    BaseObjectImpossibleSerialization,
    Vector3ImpossibleSerialization,
    Vector2ImpossibleSerialization,
    F32ImpossibleSerialization,
    RgbaImpossibleSerialization,
    BytesDeserializationIsNotImplementedYet,
    ConstMValueSliceCanOnlyBeDeserializedAsTuple,
    EnumDeserializationExpectsList { but_received: MValueType },
    EnumDeserializationExpectsListWithTwoElements,
    EnumDeserializationExpectsListOrInt { but_received: MValueType },
}

const ENUM_DESERIALIZATION_INTERNAL_TYPE: &str =
    "MValue List: [variant_index (u32), variant_value (any)]";

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dynamic_error = None;
        let info = match self {
            Error::Message(msg) => msg,
            Error::InvalidBaseObject => "Base object is destroyed and cannot be used anymore",
            Error::SerializationFailed => "Serialization failed for unknown reason",
            Error::BaseObjectImpossibleSerialization => "BaseObjectImpossibleSerialization",
            Error::BaseObjectSerializationFailed => {
                "Base object serialization failed for unknown reason"
            }
            Error::BytesDeserializationIsNotImplementedYet => {
                "Bytes deserialization is not implemented yet, use `altv::ByteBuf`"
            }
            Error::ConstMValueSliceCanOnlyBeDeserializedAsTuple => {
                "ConstMValue slice can only be deserialized as tuple"
            }
            Error::DictKeyMustBeAString => "Dict key must be a string",
            Error::DictKeySerializationFailed => "Dict key serialization failed for unknown reason",
            Error::InvalidMValueType => "InvalidMValueType",
            Error::F32ImpossibleSerialization => "F32ImpossibleSerialization",
            Error::RgbaDeserializationFailed => "Rgba deserialization failed for unknown reason",
            Error::RgbaImpossibleSerialization => "RgbaImpossibleSerialization",
            Error::RgbaSerializationFailed => "Rgba serialization failed for unknown reason",
            Error::Vector2DeserializationFailed => {
                "Vector2 deserialization failed for unknown reason"
            }
            Error::Vector2SerializationFailed => "Vector2 serialization failed for unknown reason",
            Error::Vector2ImpossibleSerialization => "Vector2ImpossibleSerialization",
            Error::Vector3DeserializationFailed => {
                "Vector3 deserialization failed for unknown reason"
            }
            Error::Vector3SerializationFailed => "Vector3 serialization failed for unknown reason",
            Error::Vector3ImpossibleSerialization => "Vector3ImpossibleSerialization",
            Error::EnumDeserializationExpectsList { but_received } => {
                let message = format!(
                    "Enum deserialization expected {ENUM_DESERIALIZATION_INTERNAL_TYPE}\
                    , received: {but_received:?}"
                );

                // i'm sorry for this
                dynamic_error.replace(message);
                dynamic_error.as_ref().unwrap()
            }
            Error::EnumDeserializationExpectsListOrInt { but_received } => {
                let message = format!(
                    "Enum deserialization expected {ENUM_DESERIALIZATION_INTERNAL_TYPE} \
                    or i32 (variant_index), received: {but_received:?}"
                );

                // i'm sorry for this
                dynamic_error.replace(message);
                dynamic_error.as_ref().unwrap()
            }
            Error::EnumDeserializationExpectsListWithTwoElements => {
                let message =
                    format!("Enum deserialization expected {ENUM_DESERIALIZATION_INTERNAL_TYPE}");
                // i'm sorry for this
                dynamic_error.replace(message);
                dynamic_error.as_ref().unwrap()
            }
        };

        f.write_str(info)
    }
}

impl std::error::Error for Error {}
