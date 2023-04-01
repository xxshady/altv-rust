use crate::{
    base_object::{AnyBaseObject, ColShapeContainer},
    helpers::{read_cpp_vector2, read_cpp_vector3},
    resource::Resource,
    vector::{Vector2, Vector3},
};
use altv_sdk::ffi as sdk;
use anyhow::Context;
use autocxx::{cxx::CxxVector, prelude::*};
use std::{collections::HashMap, fmt::Debug, ptr::NonNull};

pub struct Serializable(pub(crate) UniquePtr<sdk::MValueMutWrapper>);

macro_rules! impl_serializable {
    (@internal $value_type: ty, $create_mvalue: expr) => {
        impl TryFrom<$value_type> for Serializable {
            type Error = anyhow::Error;
            fn try_from(value: $value_type) -> anyhow::Result<Self> {
                Ok(Self(unsafe { $create_mvalue(value) }))
            }
        }
    };

    (@no_unique_ptr $value_type: ty, $create_func: expr) => {
        impl_serializable!(@internal $value_type, $create_func);
    };

    ($value_type: ty, $create_func: expr) => {
        impl_serializable!(@internal
            $value_type,
            |value| { $create_func(value) }.within_unique_ptr()
        );
    };
}

impl_serializable!(bool, sdk::create_mvalue_bool);
impl_serializable!(f64, sdk::create_mvalue_double);
impl_serializable!(i64, sdk::create_mvalue_int);
impl_serializable!(u64, sdk::create_mvalue_uint);
impl_serializable!(&str, sdk::create_mvalue_string);

impl_serializable!(Vec<Serializable>, |value| sdk::create_mvalue_list(
    convert_vec_to_mvalue_vec(value)
));

impl_serializable!(@no_unique_ptr
    HashMap<String, Serializable>,
    |value: HashMap<String, Serializable>| {
    let mut dict = sdk::create_mvalue_dict().within_unique_ptr();
        for (key, value) in value {
            sdk::push_to_mvalue_dict(dict.as_mut().unwrap(), key, value.0)
        }
        dict
    }
);

impl_serializable!(Vector3, |value: Vector3| sdk::create_mvalue_vector3(
    value.x(),
    value.y(),
    value.z()
));

impl_serializable!(Vector2, |value: Vector2| sdk::create_mvalue_vector2(
    value.x(),
    value.y(),
));

macro_rules! impl_serializable_base_object {
    ($base_object: ty, $short_name: literal) => {
        impl TryFrom<$base_object> for Serializable {
            type Error = anyhow::Error;

            fn try_from(base_object: $base_object) -> anyhow::Result<Self> {
                let mut borrowed = base_object.borrow_mut();
                let ptr = borrowed.ptr_mut();
                if ptr.valid() {
                    Ok(Self(
                        unsafe {
                            sdk::create_mvalue_base_object(
                                ptr.get().expect("this shit should never happen"),
                            )
                        }
                        .within_unique_ptr(),
                    ))
                } else {
                    anyhow::bail!("{} base object is destroyed", $short_name)
                }
            }
        }
    };
}

// TODO: fix this none/null/nil shit
/// alias for `MValue::None`
#[derive(Debug)]
pub struct MValueNone;

impl TryFrom<MValueNone> for Serializable {
    type Error = anyhow::Error;
    fn try_from(_: MValueNone) -> anyhow::Result<Self> {
        Ok(Self(
            unsafe { sdk::create_mvalue_nil() }.within_unique_ptr(),
        ))
    }
}

pub fn convert_vec_to_mvalue_vec(
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
    Vector3(Vector3),
    Vector2(Vector2),

    ColShape(ColShapeContainer),
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
    get_mvalue_type_at!(get_vector3_at, Vector3, MValue::Vector3);
    get_mvalue_type_at!(get_vector2_at, Vector2, MValue::Vector2);

    pub fn push(&mut self, mvalue: MValue) {
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
    resource: &Resource,
) -> MValueList {
    let mut deserialized_args = MValueList::default();

    for arg in args.as_ref().unwrap() {
        deserialized_args.push(deserialize_mvalue(arg, resource));
    }

    deserialized_args
}

pub(crate) fn deserialize_mvalue(cpp_wrapper: &sdk::MValueWrapper, resource: &Resource) -> MValue {
    let mvalue_type = unsafe { sdk::get_mvalue_type(cpp_wrapper) };
    let mvalue_type = altv_sdk::MValueType::from(mvalue_type).unwrap();

    use altv_sdk::MValueType::*;

    match mvalue_type {
        Bool => MValue::Bool(unsafe { sdk::get_mvalue_bool(cpp_wrapper) }),
        Double => MValue::F64(unsafe { sdk::get_mvalue_double(cpp_wrapper) }),
        String => MValue::String(unsafe { sdk::get_mvalue_string(cpp_wrapper).to_string() }),
        Nil | None => MValue::None,
        Int => MValue::I64(unsafe { sdk::get_mvalue_int(cpp_wrapper) }),
        Uint => MValue::U64(unsafe { sdk::get_mvalue_uint(cpp_wrapper) }),
        List => MValue::List(MValueList::new(
            unsafe { sdk::get_mvalue_list(cpp_wrapper) }
                .iter()
                .map(|v| deserialize_mvalue(v, resource))
                .collect(),
        )),
        Dict => MValue::Dict({
            let mut hash_map = HashMap::new();

            unsafe { sdk::get_mvalue_dict(cpp_wrapper) }
                .iter()
                .for_each(|pair| {
                    let key = unsafe { sdk::get_mvalue_dict_pair_key(pair) }.to_string();
                    let value = unsafe { sdk::get_mvalue_dict_pair_value(pair) };
                    let value =
                        deserialize_mvalue(value.within_unique_ptr().as_ref().unwrap(), resource);
                    hash_map.insert(key, value);
                });

            hash_map
        }),
        BaseObject => {
            let ptr = unsafe { sdk::get_mvalue_base_object(cpp_wrapper) };
            logger::debug!("deserializing BASE_OBJECT raw ptr: {ptr:?}");

            let Some(ptr) = NonNull::new(ptr) else {
                return MValue::InvalidBaseObject;
            };

            let base_obj = resource.base_objects.borrow().get_by_ptr(ptr);
            if let Some(base_obj) = base_obj {
                use altv_sdk::BaseObjectType::*;

                macro_rules! deserialize_base_object {
                    ($base_object_name: literal, $mvalue_item: path, $resource_impl_base_obj_map: path) => {{
                        paste::paste! {
                            let base_obj = resource
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

                match base_obj {
                    AnyBaseObject::ColShape(c) => MValue::ColShape(c),
                    unknown_base_type => {
                        logger::error!(
                            "[deserialize_mvalue] unknown baseobject type: {unknown_base_type:?}"
                        );
                        MValue::InvalidBaseObject
                    }
                }
            } else {
                logger::error!("[deserialize_mvalue] baseobject pointer is not null, but baseobject is not in pool");
                MValue::InvalidBaseObject
            }
        }
        Vector3 => MValue::Vector3(read_cpp_vector3(
            unsafe { sdk::get_mvalue_vector3(cpp_wrapper) }.within_unique_ptr(),
        )),
        Vector2 => MValue::Vector2(read_cpp_vector2(
            unsafe { sdk::get_mvalue_vector2(cpp_wrapper) }.within_unique_ptr(),
        )),
        _ => {
            logger::error!("[deserialize_mvalue] unknown mvalue type: {mvalue_type:?}");
            MValue::None
        }
    }
}

#[macro_export]
macro_rules! serialize_mvalue {
    ($value: expr, $vec: expr) => {
        let serializable = $crate::mvalue::Serializable::try_from($value);

        match serializable {
            Ok(serialized) => {
                $vec.push(serialized);
            }
            Err(error) => {
                $crate::anyhow::bail!(
                    "Failed to convert value: {} to mvalue, error: {}",
                    stringify!($value),
                    error
                );
            }
        }
    };
}

#[macro_export]
macro_rules! mvalue_list {
    ($($arg: expr),+ $(,)*) => {
        (|| {
            let mut vec = vec![];
            $(
                $crate::serialize_mvalue!($arg, vec);
            )+
            Ok(vec)
        })()
    };
}

#[macro_export]
macro_rules! mvalue_dict {
    ($($key: expr => $value: expr),+ $(,)*) => {
        (||{
            let mut hash_map = std::collections::HashMap::new();
            $(
                let serializable = $crate::mvalue::Serializable::try_from($value);
                match serializable {
                    Ok(serialized) => {
                        hash_map.insert($key.to_string(), serialized);
                    }
                    Err(error) => {
                        $crate::anyhow::bail!(
                            "Failed to convert value: {} to mvalue, error: {}",
                            stringify!($value),
                            error
                        );
                    }
                }
            )+
            Ok(hash_map)
        })()
    };
}
