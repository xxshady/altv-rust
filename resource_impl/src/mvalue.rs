use std::collections::HashMap;

use crate::mvalue;
use altv_sdk::ffi as sdk;
use anyhow::Context;
use autocxx::{cxx::CxxVector, prelude::*};

pub struct Serializable(UniquePtr<sdk::MValueWrapper>);

impl From<bool> for Serializable {
    fn from(value: bool) -> Self {
        Self(unsafe { sdk::create_mvalue_bool(value) }.within_unique_ptr())
    }
}

impl From<f64> for Serializable {
    fn from(value: f64) -> Self {
        Self(unsafe { sdk::create_mvalue_double(value) }.within_unique_ptr())
    }
}

impl From<&str> for Serializable {
    fn from(value: &str) -> Self {
        Self(unsafe { sdk::create_mvalue_string(value) }.within_unique_ptr())
    }
}

// TODO: fix this none/null/nil shit
/// alias for `MValue::None`
#[derive(Debug)]
pub struct None;

impl From<None> for Serializable {
    fn from(_: None) -> Self {
        Self(unsafe { sdk::create_mvalue_nil() }.within_unique_ptr())
    }
}

impl From<i64> for Serializable {
    fn from(value: i64) -> Self {
        Self(unsafe { sdk::create_mvalue_int(value) }.within_unique_ptr())
    }
}

impl From<u64> for Serializable {
    fn from(value: u64) -> Self {
        Self(unsafe { sdk::create_mvalue_uint(value) }.within_unique_ptr())
    }
}

impl From<Vec<Serializable>> for Serializable {
    fn from(value: Vec<Serializable>) -> Self {
        Self(
            unsafe { sdk::create_mvalue_list(convert_vec_to_mvalue_vec(value)) }
                .within_unique_ptr(),
        )
    }
}

impl From<HashMap<String, Serializable>> for Serializable {
    fn from(value: HashMap<String, Serializable>) -> Self {
        Self({
            let mut dict = unsafe { sdk::create_mvalue_dict() }.within_unique_ptr();
            for (key, value) in value {
                unsafe { sdk::push_to_mvalue_dict(dict.as_mut().unwrap(), key, value.0) }
            }
            unsafe { sdk::convert_mvalue_mut_wrapper_to_const(dict) }.within_unique_ptr()
        })
    }
}

pub(crate) fn convert_vec_to_mvalue_vec(
    vec: Vec<Serializable>,
) -> UniquePtr<CxxVector<sdk::MValueWrapper>> {
    let mut mvalue_vec = unsafe { sdk::create_mvalue_vec() };

    for value in vec {
        unsafe {
            sdk::push_to_mvalue_vec(mvalue_vec.as_mut().unwrap(), value.0);
        }
    }

    mvalue_vec
}

#[derive(Debug)]
pub enum MValue {
    Bool(bool),
    F64(f64),
    String(String),
    None,
    I64(i64),
    U64(u64),
    List(MValueList),
    Dict(HashMap<String, MValue>),
}

macro_rules! get_mvalue_type_at {
    ($method_name: ident, $type_name: ty, $mvalue_type: path) => {
        pub fn $method_name(&self, index: usize) -> anyhow::Result<&$type_name> {
            let value = self.get(index).with_context(|| {
                format!(
                    "MValueArgs {} index: {index} does not exists",
                    stringify!($method_name),
                )
            })?;

            if let $mvalue_type(value) = value {
                Ok(&value)
            } else {
                anyhow::bail!(
                    "MValueArgs {} index: {index} exists but is not: {} (it is actually: {:?})",
                    stringify!($method_name),
                    stringify!($type_name),
                    value
                )
            }
        }
    };
}

#[derive(Debug, Default)]
pub struct MValueList {
    vec: Vec<MValue>,
}

impl MValueList {
    pub fn new(vec: Vec<MValue>) -> Self {
        Self { vec }
    }

    pub fn get(&self, index: usize) -> anyhow::Result<&MValue> {
        self.vec
            .get(index)
            .ok_or(anyhow::anyhow!("MValueArgs get({index}) does not exists"))
    }

    get_mvalue_type_at!(get_bool_at, bool, MValue::Bool);
    get_mvalue_type_at!(get_f64_at, f64, MValue::F64);
    get_mvalue_type_at!(get_string_at, String, MValue::String);
    get_mvalue_type_at!(get_i64_at, i64, MValue::I64);
    get_mvalue_type_at!(get_u64_at, u64, MValue::U64);
    get_mvalue_type_at!(get_list_at, MValueList, MValue::List);
    get_mvalue_type_at!(get_dict_at, HashMap<String, MValue>, MValue::Dict);

    pub(crate) fn push(&mut self, mvalue: MValue) {
        self.vec.push(mvalue);
    }
}

pub fn deserialize_mvalue_args(args: UniquePtr<CxxVector<sdk::MValueWrapper>>) -> MValueList {
    let mut deserialized_args = MValueList::default();

    for arg in args.as_ref().unwrap() {
        deserialized_args.push(deserialize_mvalue(arg));
    }

    deserialized_args
}

fn deserialize_mvalue(cpp_wrapper: &sdk::MValueWrapper) -> MValue {
    let mvalue_type = unsafe { sdk::get_mvalue_type(cpp_wrapper) };
    let mvalue_type = altv_sdk::MValueType::from(mvalue_type).unwrap();

    use altv_sdk::MValueType::*;

    match mvalue_type {
        BOOL => mvalue::MValue::Bool(unsafe { sdk::get_mvalue_bool(cpp_wrapper) }),
        DOUBLE => mvalue::MValue::F64(unsafe { sdk::get_mvalue_double(cpp_wrapper) }),
        STRING => {
            mvalue::MValue::String(unsafe { sdk::get_mvalue_string(cpp_wrapper).to_string() })
        }
        NIL | NONE => mvalue::MValue::None,
        INT => mvalue::MValue::I64(unsafe { sdk::get_mvalue_int(cpp_wrapper) }),
        UINT => mvalue::MValue::U64(unsafe { sdk::get_mvalue_uint(cpp_wrapper) }),
        LIST => mvalue::MValue::List(MValueList::new(
            unsafe { sdk::get_mvalue_list(cpp_wrapper) }
                .iter()
                .map(deserialize_mvalue)
                .collect(),
        )),
        DICT => mvalue::MValue::Dict({
            let mut hash_map = HashMap::new();

            unsafe { sdk::get_mvalue_dict(cpp_wrapper) }
                .iter()
                .for_each(|pair| {
                    let key = unsafe { sdk::get_mvalue_dict_pair_key(pair) }.to_string();
                    let value = unsafe { sdk::get_mvalue_dict_pair_value(pair) };
                    let value = deserialize_mvalue(value.within_unique_ptr().as_ref().unwrap());
                    hash_map.insert(key, value);
                });

            hash_map
        }),
        _ => {
            logger::error!("[deserialize_mvalue] unknown mvalue type: {mvalue_type:?}");
            mvalue::MValue::None
        }
    }
}

#[macro_export]
macro_rules! mvalue_list {
    ($($arg: expr),+ $(,)*) => {
        vec![$($arg.into()),*]
    };
}

#[macro_export]
macro_rules! mvalue_dict {
    ($($key: expr => $value: expr),+ $(,)*) => {{
        let mut hash_map = std::collections::HashMap::new();
        $( hash_map.insert($key.to_string(), $value.into()); )+
        hash_map
    }};
}
