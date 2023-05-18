use std::dbg;

use serde::{
    ser::{self, Impossible},
    Serialize,
};

use crate::{ser::MValue, serialize_simple, Error, Result};

use altv_sdk::ffi as sdk;

pub const BASE_OBJECT_MVALUE: &str = "___altv_base_object_mvalue";

pub struct BaseObjectSerializer {
    output: Option<MValue>,
}

pub fn to_base_object_mvalue<T>(value: &T) -> Result<MValue>
where
    T: Serialize + ?Sized,
{
    let mut serializer = BaseObjectSerializer { output: None };
    value.serialize(&mut serializer)?;
    serializer
        .output
        .ok_or(Error::BaseObjectSerializationFailed)
}

impl<'a> ser::Serializer for &'a mut BaseObjectSerializer {
    type Ok = ();
    type Error = Error;

    // see `Serialize` impl in core_resource/src/base_objects/base_impl/wrapper.rs
    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        let base_object_ptr = v as altv_sdk::BaseObjectRawMutPtr;
        if base_object_ptr.is_null() {
            return Err(Error::InvalidBaseObject);
        }

        serialize_simple!(self, sdk::create_mvalue_base_object(base_object_ptr))
    }

    ///////////////////////////////////////////////////////////////////////////////////////////

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_bytes(self, base_object_ptr: &[u8]) -> Result<Self::Ok> {
        dbg!();
        let bytes = base_object_ptr.split_at(8).0.try_into().unwrap();
        dbg!(&bytes);
        let usize_bytes = usize::from_le_bytes(bytes);
        dbg!(&usize_bytes);

        serialize_simple!(
            self,
            sdk::create_mvalue_base_object(usize_bytes as altv_sdk::BaseObjectRawMutPtr)
        )
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_some<T: ?Sized>(self, _v: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        dbg!();
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _v: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(Error::BaseObjectImpossibleSerialization)
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
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        dbg!();
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        dbg!();
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _base_object_ptr: usize,
    ) -> Result<Self::SerializeStruct> {
        Err(Error::BaseObjectImpossibleSerialization)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::BaseObjectImpossibleSerialization)
    }
}
