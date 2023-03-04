use crate::{
    base_object::{self, BaseObject},
    player,
    resource_impl::ResourceImpl,
    vehicle,
};
use altv_sdk::ffi as sdk;
use anyhow::Context;
use autocxx::{cxx::CxxVector, prelude::*};
use std::{cell::RefMut, collections::HashMap, fmt::Debug};

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

pub struct ValidBaseObject<'a>(RefMut<'a, dyn base_object::BaseObject>);

macro_rules! impl_base_object_to_serializable {
    ($base_object_struct: path, $container: path, $func_name: ident) => {
        impl<'a> TryFrom<RefMut<'a, $base_object_struct>> for ValidBaseObject<'a> {
            type Error = anyhow::Error;

            fn try_from(value: RefMut<'a, $base_object_struct>) -> Result<Self, Self::Error> {
                if !value.valid() {
                    anyhow::bail!(
                        "base object: {} is invalid",
                        stringify!($base_object_struct)
                    );
                }
                Ok(Self(value))
            }
        }

        impl From<$container> for Serializable {
            fn from(value: $container) -> Self {
                Self(
                    unsafe {
                        sdk::create_mvalue_base_object(
                            value
                                .borrow_mut()
                                .ptr_mut()
                                .get()
                                // this should never panic since ValidBaseObject protects baseobject from being destroyed
                                .expect("this shit should never happen"),
                        )
                    }
                    .within_unique_ptr(),
                )
            }
        }

        pub fn $func_name<'a>(
            base_object: RefMut<'a, $base_object_struct>,
        ) -> Result<ValidBaseObject<'a>, anyhow::Error> {
            $crate::mvalue::ValidBaseObject::try_from(base_object)
        }
    };
}

impl_base_object_to_serializable!(vehicle::Vehicle, vehicle::VehicleContainer, valid_vehicle);
impl_base_object_to_serializable!(player::Player, player::PlayerContainer, valid_player);

impl From<ValidBaseObject<'_>> for Serializable {
    fn from(mut value: ValidBaseObject) -> Self {
        Self(
            unsafe { sdk::create_mvalue_base_object(value.0.ptr_mut().get().unwrap()) }
                .within_unique_ptr(),
        )
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
    Vehicle(vehicle::VehicleContainer),
    Player(player::PlayerContainer),
    InvalidBaseObject,
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

#[derive(Default)]
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

impl Debug for MValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.vec)
    }
}

pub fn deserialize_mvalue_args(
    args: UniquePtr<CxxVector<sdk::MValueWrapper>>,
    resource_impl: &ResourceImpl,
) -> MValueList {
    let mut deserialized_args = MValueList::default();

    for arg in args.as_ref().unwrap() {
        deserialized_args.push(deserialize_mvalue(arg, resource_impl));
    }

    deserialized_args
}

fn deserialize_mvalue(cpp_wrapper: &sdk::MValueWrapper, resource_impl: &ResourceImpl) -> MValue {
    let mvalue_type = unsafe { sdk::get_mvalue_type(cpp_wrapper) };
    let mvalue_type = altv_sdk::MValueType::from(mvalue_type).unwrap();

    use altv_sdk::MValueType::*;

    match mvalue_type {
        BOOL => MValue::Bool(unsafe { sdk::get_mvalue_bool(cpp_wrapper) }),
        DOUBLE => MValue::F64(unsafe { sdk::get_mvalue_double(cpp_wrapper) }),
        STRING => MValue::String(unsafe { sdk::get_mvalue_string(cpp_wrapper).to_string() }),
        NIL | NONE => MValue::None,
        INT => MValue::I64(unsafe { sdk::get_mvalue_int(cpp_wrapper) }),
        UINT => MValue::U64(unsafe { sdk::get_mvalue_uint(cpp_wrapper) }),
        LIST => MValue::List(MValueList::new(
            unsafe { sdk::get_mvalue_list(cpp_wrapper) }
                .iter()
                .map(|v| deserialize_mvalue(v, resource_impl))
                .collect(),
        )),
        DICT => MValue::Dict({
            let mut hash_map = HashMap::new();

            unsafe { sdk::get_mvalue_dict(cpp_wrapper) }
                .iter()
                .for_each(|pair| {
                    let key = unsafe { sdk::get_mvalue_dict_pair_key(pair) }.to_string();
                    let value = unsafe { sdk::get_mvalue_dict_pair_value(pair) };
                    let value = deserialize_mvalue(
                        value.within_unique_ptr().as_ref().unwrap(),
                        resource_impl,
                    );
                    hash_map.insert(key, value);
                });

            hash_map
        }),
        BASE_OBJECT => {
            let raw_ptr = unsafe { sdk::get_mvalue_base_object(cpp_wrapper) };
            logger::debug!("deserializing BASE_OBJECT raw ptr: {raw_ptr:?}");
            if raw_ptr.is_null() {
                return MValue::InvalidBaseObject;
            }

            let base_obj = resource_impl.borrow_base_objects().get_by_raw_ptr(raw_ptr);
            if let Some(base_obj) = base_obj {
                use altv_sdk::BaseObjectType::*;

                macro_rules! deserialize_base_object {
                    ($base_object_name: literal, $mvalue_item: path, $resource_impl_base_obj_map: path) => {{
                        paste::paste! {
                            let base_obj = resource_impl
                            .[<$resource_impl_base_obj_map>]
                            .borrow()
                            .get_by_base_object_ptr(raw_ptr);

                        if let Some(base_obj) = base_obj {
                            MValue::$mvalue_item(base_obj)
                        } else {
                            logger::error!(
                                "[deserialize_mvalue] {0} baseobject pointer is not null, but {0} is not in pool",
                                stringify!($base_object_name)
                            );
                            MValue::InvalidBaseObject
                        }
                        }
                    }};
                }

                match base_obj.borrow().base_type() {
                    VEHICLE => {
                        deserialize_base_object!("vehicle", Vehicle, vehicle_base_object_map)
                    }
                    PLAYER => {
                        deserialize_base_object!("player", Player, player_base_object_map)
                    }
                    base_type => {
                        logger::error!(
                            "[deserialize_mvalue] unknown baseobject type: {base_type:?}"
                        );
                        MValue::InvalidBaseObject
                    }
                }
            } else {
                logger::error!("[deserialize_mvalue] baseobject pointer is not null, but baseobject is not in pool");
                MValue::InvalidBaseObject
            }
        }
        _ => {
            logger::error!("[deserialize_mvalue] unknown mvalue type: {mvalue_type:?}");
            MValue::None
        }
    }
}

#[macro_export]
macro_rules! mvalue_list {
    ($($arg: expr),+ $(,)*) => {
        vec![$(
            $crate::mvalue::Serializable::from($arg)
        ),*]
    };
}

#[macro_export]
macro_rules! mvalue_dict {
    ($($key: expr => $value: expr),+ $(,)*) => {{
        let mut hash_map = std::collections::HashMap::new();
        $(
            hash_map.insert($key.to_string(), $crate::mvalue::Serializable::from($value));
        )+
        hash_map
    }};
}
