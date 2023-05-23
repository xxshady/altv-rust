use crate::{ConstMValue, Deserializer, Error, Result};

use serde::{de, forward_to_deserialize_any, Deserialize};

struct ConstMValueSliceDeserializer<'de> {
    input: &'de [ConstMValue],
}

impl<'de> ConstMValueSliceDeserializer<'de> {
    pub fn from_mvalue_slice(input: &'de [ConstMValue]) -> Self {
        ConstMValueSliceDeserializer { input }
    }
}

pub fn from_mvalue_slice<'de, T>(m: &'de [ConstMValue]) -> Result<T>
where
    T: Deserialize<'de>,
{
    let deserializer = ConstMValueSliceDeserializer::from_mvalue_slice(m);
    let t = T::deserialize(deserializer)?;
    Ok(t)
}

impl<'de> de::Deserializer<'de> for ConstMValueSliceDeserializer<'de> {
    type Error = Error;

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(Seq::new(self.input))
    }

    /////////////////////////////////////////////////////////////////////////////

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::ConstMValueSliceCanOnlyBeDeserializedAsTuple)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq
        tuple_struct map struct enum identifier ignored_any
    }
}

struct Seq<'de> {
    slice: &'de [ConstMValue],
    current_idx: usize,
}

impl<'de> Seq<'de> {
    fn new(slice: &'de [ConstMValue]) -> Self {
        Self {
            slice,
            current_idx: 0,
        }
    }

    fn next(&mut self) -> Option<ConstMValue> {
        if self.current_idx == self.slice.len() {
            return None;
        }
        let value = self.slice.get(self.current_idx);
        self.current_idx += 1;
        value.cloned()
    }
}

impl<'de> de::SeqAccess<'de> for Seq<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        let next = self.next();
        let Some(mvalue) = next else {
            return Ok(None);
        };
        let mut deserializer = Deserializer::from_mvalue(mvalue);
        seed.deserialize(&mut deserializer).map(Some)
    }
}

pub trait DeserializeMValueArgs {
    fn deserialize<'de, V: Deserialize<'de>>(&'de self) -> Result<V>;
}

impl DeserializeMValueArgs for Vec<ConstMValue> {
    fn deserialize<'de, V: Deserialize<'de>>(&'de self) -> Result<V> {
        from_mvalue_slice(self)
    }
}
