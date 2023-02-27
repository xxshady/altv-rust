use crate::mvalue;
use altv_sdk::ffi::{self as sdk, MValueWrapper};
use anyhow::Context;
use autocxx::{cxx::CxxVector, prelude::*};

pub fn create_mvalue_bool(value: bool) -> UniquePtr<sdk::MValueWrapper> {
    unsafe { sdk::create_mvalue_bool(value).within_unique_ptr() }
}

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
                    "MValueArgs {} index: {index} exists but is not {}",
                    stringify!($method_name),
                    stringify!($type_name),
                )
            }
        }
    };
}

#[derive(Debug)]
pub struct MValueArgs {
    vec: Vec<MValue>,
}

impl MValueArgs {
    pub fn new() -> Self {
        Self { vec: vec![] }
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

    pub(crate) fn push(&mut self, mvalue: MValue) {
        self.vec.push(mvalue);
    }
}

pub fn deserialize_mvalue_args(args: UniquePtr<CxxVector<MValueWrapper>>) -> MValueArgs {
    let mut deserialized_args = mvalue::MValueArgs::new();

    for arg in args.as_ref().unwrap() {
        let mvalue_type = unsafe { sdk::get_mvalue_wrapper_type(arg) };
        let mvalue_type = altv_sdk::MValueType::from(mvalue_type).unwrap();

        use altv_sdk::MValueType::*;
        let mvalue = match mvalue_type {
            BOOL => mvalue::MValue::Bool(unsafe { sdk::get_mvalue_wrapper_bool(arg) }),
            DOUBLE => mvalue::MValue::F64(unsafe { sdk::get_mvalue_wrapper_double(arg) }),
            STRING => {
                mvalue::MValue::String(unsafe { sdk::get_mvalue_wrapper_string(arg).to_string() })
            }
            NIL | NONE => mvalue::MValue::None,
            INT => mvalue::MValue::I64(unsafe { sdk::get_mvalue_wrapper_int(arg) }),
            UINT => mvalue::MValue::U64(unsafe { sdk::get_mvalue_wrapper_uint(arg) }),
            _ => todo!(),
        };
        deserialized_args.push(mvalue);
    }

    deserialized_args
}
