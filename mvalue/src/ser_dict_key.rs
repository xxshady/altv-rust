use serde::{
    ser::{self, Impossible},
    Serialize,
};

use crate::{helpers::serialize_simple, wrappers::MutMValue, Error, Result};

use altv_sdk::ffi as sdk;

pub struct DictKeySerializer {
    output: Option<MutMValue>,
}

pub fn to_dict_key_mvalue<T>(value: &T) -> Result<MutMValue>
where
    T: Serialize + ?Sized,
{
    let mut serializer = DictKeySerializer { output: None };
    value.serialize(&mut serializer)?;
    serializer.output.ok_or(Error::DictKeySerializationFailed)
}

impl<'a> ser::Serializer for &'a mut DictKeySerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        serialize_simple!(self, sdk::create_mvalue_string(v))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_some<T: ?Sized>(self, _v: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _v: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(Error::DictKeyMustBeAString)
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
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::DictKeyMustBeAString)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::DictKeyMustBeAString)
    }
}
