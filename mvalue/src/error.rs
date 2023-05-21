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
    Vector2SerializationFailed,
    RgbaSerializationFailed,
    InvalidBaseObject,
    BaseObjectNotFound,
    InvalidMValueType,
    DictKeyMustBeAString,
    BaseObjectImpossibleSerialization,
    Vector3ImpossibleSerialization,
    Vector2ImpossibleSerialization,
    F32ImpossibleSerialization,
    RgbaImpossibleSerialization,
    UnimplementedMValueType(MValueType),
}

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
        // TODO:
        write!(f, "MValue error: {:?}", self)
    }
}

impl std::error::Error for Error {}
