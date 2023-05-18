use autocxx::{cxx::CxxString, prelude::*};
use serde::{
    de::{self, Visitor},
    forward_to_deserialize_any,
};

use crate::{Error, Result};

pub struct DictKeyDeserializer {
    input: UniquePtr<CxxString>,
}

impl DictKeyDeserializer {
    pub fn from_cpp_string(input: UniquePtr<CxxString>) -> Self {
        DictKeyDeserializer { input }
    }
}

impl<'de> de::Deserializer<'de> for DictKeyDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.input.to_string())
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
