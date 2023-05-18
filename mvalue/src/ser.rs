use altv_sdk::ffi as sdk;
use autocxx::prelude::*;
use serde::{ser, Serialize};

use crate::{
    error::{Error, Result},
    ser_base_object::{to_base_object_mvalue, BASE_OBJECT_MVALUE},
    ser_dict_key::to_dict_key_mvalue,
    serialize_simple,
    types::RawMutMValue,
};

pub struct MValue(pub RawMutMValue);

pub struct Serializer {
    output: Option<MValue>,
}

pub fn to_mvalue<T>(value: &T) -> Result<MValue>
where
    T: Serialize + ?Sized,
{
    let mut serializer = Serializer { output: None };
    value.serialize(&mut serializer)?;
    serializer.output.ok_or(Error::SerializationFailed)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = MValueList;
    type SerializeTuple = MValueList;
    type SerializeTupleStruct = MValueList;
    type SerializeTupleVariant = MValueList;
    type SerializeMap = MValueDict;
    type SerializeStruct = MValueDict;
    type SerializeStructVariant = MValueDict;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        serialize_simple!(self, sdk::create_mvalue_bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        serialize_simple!(self, sdk::create_mvalue_int(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        serialize_simple!(self, sdk::create_mvalue_uint(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        serialize_simple!(self, sdk::create_mvalue_double(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        serialize_simple!(self, sdk::create_mvalue_string(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        serialize_simple!(self, sdk::create_mvalue_byte_array(v.as_ptr(), v.len()))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        serialize_simple!(self, sdk::create_mvalue_nil())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        self.output = Some(to_mvalue(value)?);
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_u32(variant_index)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        self.output = Some(match name {
            BASE_OBJECT_MVALUE => to_base_object_mvalue(value)?,
            _ => to_mvalue(value)?,
        });
        Ok(())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        self.output = Some(to_mvalue(value)?);
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        let list = MValueList::new();
        self.output = Some(MValue(
            unsafe { sdk::copy_mut_mvalue(list.mvalue.0.as_ref().unwrap()) }.within_unique_ptr(),
        ));
        Ok(list)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.serialize_tuple(len)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        let dict = MValueDict::new();
        self.output = Some(MValue(
            unsafe { sdk::copy_mut_mvalue(dict.mvalue.0.as_ref().unwrap()) }.within_unique_ptr(),
        ));
        Ok(dict)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.serialize_map(Some(len))
    }
}

pub struct MValueDict {
    mvalue: MValue,
}

impl MValueDict {
    fn new() -> Self {
        Self {
            mvalue: MValue(unsafe { sdk::create_mvalue_dict().within_unique_ptr() }),
        }
    }
}

impl ser::SerializeMap for MValueDict {
    type Ok = ();
    type Error = Error;

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<()>
    where
        K: Serialize + ?Sized,
        V: Serialize + ?Sized,
    {
        let key = to_dict_key_mvalue(key)?;
        let value = to_mvalue(value)?;

        unsafe { sdk::push_to_mvalue_dict(self.mvalue.0.as_mut().unwrap(), key.0, value.0) };

        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }
}

impl ser::SerializeStruct for MValueDict {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        <Self as ser::SerializeMap>::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<()> {
        <Self as ser::SerializeMap>::end(self)
    }
}

impl ser::SerializeStructVariant for MValueDict {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        <Self as ser::SerializeMap>::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<()> {
        <Self as ser::SerializeMap>::end(self)
    }
}

pub struct MValueList {
    mvalue: MValue,
}

impl MValueList {
    fn new() -> Self {
        Self {
            mvalue: MValue(unsafe { sdk::create_mvalue_list().within_unique_ptr() }),
        }
    }
}

impl ser::SerializeSeq for MValueList {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unsafe {
            sdk::push_to_mvalue_list(
                self.mvalue.0.as_mut().unwrap(),
                sdk::convert_mvalue_mut_wrapper_to_const(to_mvalue(value)?.0).within_unique_ptr(),
            )
        };

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl ser::SerializeTuple for MValueList {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        <Self as ser::SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        <Self as ser::SerializeSeq>::end(self)
    }
}

impl ser::SerializeTupleStruct for MValueList {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        <Self as ser::SerializeTuple>::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        <Self as ser::SerializeTuple>::end(self)
    }
}

impl ser::SerializeTupleVariant for MValueList {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        <Self as ser::SerializeTuple>::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        <Self as ser::SerializeTuple>::end(self)
    }
}
