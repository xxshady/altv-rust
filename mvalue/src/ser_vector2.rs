use serde::{
    ser::{self, Impossible},
    Serialize,
};

use crate::{bytes_num, helpers::serialize_simple, wrappers::MutMValue, Error, Result};

use altv_sdk::ffi as sdk;

pub const VECTOR2_MVALUE: &str = "___altv_vector2_mvalue";

pub struct Vector2Serializer {
    output: Option<MutMValue>,
}

pub fn to_vector2_mvalue<T>(value: &T) -> Result<MutMValue>
where
    T: Serialize + ?Sized,
{
    let mut serializer = Vector2Serializer { output: None };
    value.serialize(&mut serializer)?;
    serializer.output.ok_or(Error::Vector2SerializationFailed)
}

impl<'a> ser::Serializer for &'a mut Vector2Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        let Some([x, y]) = bytes_num::from_byte_buf::<f32, 4, 2>(v) else {
            return Err(Error::Vector2SerializationFailed);
        };
        serialize_simple!(self, sdk::create_mvalue_vector2(x, y))
    }

    ///////////////////////////////////////////////////////////////////////////////////////////

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_some<T: ?Sized>(self, _v: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _v: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _v: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::Vector2ImpossibleSerialization)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::Vector2ImpossibleSerialization)
    }
}
