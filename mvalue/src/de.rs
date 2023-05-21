use altv_sdk::{
    ffi::{self as sdk},
    MValueType,
};
use autocxx::{cxx::CxxVector, prelude::*};
use serde::{
    de::{self, DeserializeOwned, DeserializeSeed, MapAccess, SeqAccess, Visitor},
    forward_to_deserialize_any,
};

use crate::{
    bytes_num,
    de_dict_key::DictKeyDeserializer,
    wrappers::{ConstMValue, MutMValue},
    Error, Result,
};

pub struct Deserializer {
    input: ConstMValue,
}

impl Deserializer {
    pub fn from_mvalue(input: ConstMValue) -> Self {
        Deserializer { input }
    }
}

pub fn from_mvalue<T>(m: MutMValue) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut deserializer = Deserializer::from_mvalue(m.into_const());
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

impl Deserializer {
    fn mvalue_type(&self) -> Result<MValueType> {
        let raw = unsafe { sdk::read_mvalue_type(self.input.get()) };
        MValueType::try_from(raw).map_err(|_| Error::InvalidMValueType)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mvalue = self.input.get();

        match self.mvalue_type()? {
            MValueType::Bool => visitor.visit_bool(unsafe { sdk::read_mvalue_bool(mvalue) }),
            MValueType::Int => visitor.visit_i64(unsafe { sdk::read_mvalue_int(mvalue) }),
            MValueType::Uint => visitor.visit_u64(unsafe { sdk::read_mvalue_uint(mvalue) }),
            MValueType::Double => visitor.visit_f64(unsafe { sdk::read_mvalue_double(mvalue) }),
            MValueType::String => {
                visitor.visit_string(unsafe { sdk::read_mvalue_string(mvalue) }.to_string())
            }
            MValueType::ByteArray => {
                let size = unsafe { sdk::read_mvalue_byte_array_size(mvalue) };
                let mut buffer = Vec::<u8>::with_capacity(size);
                unsafe {
                    sdk::read_mvalue_byte_array(mvalue, buffer.as_mut_ptr());
                    buffer.set_len(size);
                }
                visitor.visit_byte_buf(buffer)
            }
            MValueType::BaseObject => {
                let raw_ptr = unsafe { sdk::read_mvalue_base_object(mvalue) };
                visitor.visit_u64(raw_ptr as u64)
            }
            MValueType::Vector3 => {
                let ptr = unsafe { sdk::read_mvalue_vector3(mvalue) }.within_unique_ptr();
                let mut x = 0f32;
                let mut y = 0f32;
                let mut z = 0f32;
                unsafe {
                    sdk::read_vector3(
                        ptr.as_ref().unwrap(),
                        &mut x as *mut f32,
                        &mut y as *mut f32,
                        &mut z as *mut f32,
                    );
                }

                let buf = bytes_num::to_byte_buf([x, y, z]);
                visitor.visit_byte_buf(buf)
            }
            MValueType::Vector2 => {
                let ptr = unsafe { sdk::read_mvalue_vector2(mvalue) }.within_unique_ptr();
                let mut x = 0f32;
                let mut y = 0f32;
                unsafe {
                    sdk::read_vector2(
                        ptr.as_ref().unwrap(),
                        &mut x as *mut f32,
                        &mut y as *mut f32,
                    );
                }

                let buf = bytes_num::to_byte_buf([x, y]);
                visitor.visit_byte_buf(buf)
            }
            MValueType::List => {
                visitor.visit_seq(Seq::new(unsafe { sdk::read_mvalue_list(self.input.get()) }))
            }
            MValueType::Rgba => {
                let ptr = unsafe { sdk::read_mvalue_rgba(mvalue) }.within_unique_ptr();
                let mut r = 0u8;
                let mut g = 0u8;
                let mut b = 0u8;
                let mut a = 0u8;
                unsafe {
                    sdk::read_rgba(
                        ptr.as_ref().unwrap(),
                        &mut r as *mut _,
                        &mut g as *mut _,
                        &mut b as *mut _,
                        &mut a as *mut _,
                    );
                }

                let buf = bytes_num::to_byte_buf([r, g, b, a]);
                visitor.visit_byte_buf(buf)
            }

            unimplemented_type => Err(Error::UnimplementedMValueType(unimplemented_type)),
        }
    }

    forward_to_deserialize_any! {
        bool
        i8 i16 i32 i64
        u8 u16 u32 u64
        f32 f64
        char str string
        bytes // TODO:
        byte_buf
        unit unit_struct
        enum identifier ignored_any
        seq
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.mvalue_type()? {
            MValueType::None | MValueType::Nil => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    // TODO: forward it to any
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // TODO: forward it to any?
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // TODO: forward it to any?
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // TODO: forward it to any
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(Map::new(unsafe { sdk::read_mvalue_dict(self.input.get()) }))
    }

    // TODO: forward it to any?
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }
}

struct Seq {
    mvalue_list: UniquePtr<CxxVector<sdk::ConstMValueWrapper>>,
    current_idx: usize,
}

impl Seq {
    fn new(mvalue_list: UniquePtr<CxxVector<sdk::ConstMValueWrapper>>) -> Self {
        Self {
            mvalue_list,
            current_idx: 0,
        }
    }

    fn next(&mut self) -> Option<ConstMValue> {
        if self.current_idx == self.mvalue_list.len() {
            return None;
        }
        let value = self.mvalue_list.get(self.current_idx);
        let value = value
            .map(|v| ConstMValue::new(unsafe { sdk::copy_const_mvalue(v) }.within_unique_ptr()));

        self.current_idx += 1;
        value
    }
}

impl<'de> SeqAccess<'de> for Seq {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let next = self.next();
        let Some(mvalue) = next else {
            return Ok(None);
        };
        let mut deserializer = Deserializer::from_mvalue(mvalue);
        seed.deserialize(&mut deserializer).map(Some)
    }
}

struct Map {
    mvalue_dict: UniquePtr<CxxVector<sdk::MValueDictPairWrapper>>,
    current_idx: usize,
    current_pair: Option<UniquePtr<sdk::MValueDictPairWrapper>>,
}

impl Map {
    fn new(mvalue_dict: UniquePtr<CxxVector<sdk::MValueDictPairWrapper>>) -> Self {
        Self {
            mvalue_dict,
            current_idx: 0,
            current_pair: None,
        }
    }

    fn next(&mut self) -> &Option<UniquePtr<sdk::MValueDictPairWrapper>> {
        if self.current_idx == self.mvalue_dict.len() {
            return &None;
        }
        let value = self.mvalue_dict.get(self.current_idx);
        self.current_idx += 1;
        self.current_pair =
            value.map(|v| unsafe { sdk::copy_mvalue_dict_pair(v) }.within_unique_ptr());
        &self.current_pair
    }

    fn current_pair_value(&self) -> ConstMValue {
        let pair = self.current_pair.as_ref().unwrap();
        ConstMValue::new(
            unsafe { sdk::read_mvalue_dict_pair_value(pair.as_ref().unwrap()) }.within_unique_ptr(),
        )
    }
}

// for some reason serde Deserialize macro uses key & value functions, not entry
impl<'de> MapAccess<'de> for Map {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        let next = self.next();
        let Some(pair) = next else {
            return Ok(None);
        };
        let key = unsafe { sdk::read_mvalue_dict_pair_key(pair.as_ref().unwrap()) };
        let key = seed.deserialize(DictKeyDeserializer::from_cpp_string(key))?;
        Ok(Some(key))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let value = self.current_pair_value();
        let mut deserializer = Deserializer::from_mvalue(value);
        let value = seed.deserialize(&mut deserializer)?;
        Ok(value)
    }
}
