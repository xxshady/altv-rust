use serde::{
    de::{self, Visitor},
    forward_to_deserialize_any,
};

use crate::{Error, Result};

pub struct EnumVariantDeserializer {
    variant_index: u32,
}

impl EnumVariantDeserializer {
    pub fn from_cpp(input: u32) -> Self {
        EnumVariantDeserializer {
            variant_index: input,
        }
    }
}

impl<'de> de::Deserializer<'de> for EnumVariantDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.variant_index as u64)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
