use crate::{
    base_objects::{
        blip, checkpoint, col_shape, marker, network_object, ped, player, vehicle, virtual_entity,
        virtual_entity_group, voice_channel, AnyBaseObject, BasePtr,
    },
    helpers::{read_cpp_vector2, read_cpp_vector3},
    resource::Resource,
    vector::{Vector2, Vector3},
    SomeResult,
};
use altv_sdk::{ffi as sdk, helpers::get_base_object_type};
use anyhow::Context;
use autocxx::{cxx::CxxVector, prelude::*};
use std::{collections::HashMap, fmt::Debug, ptr::NonNull};

pub struct Serializable(pub(crate) UniquePtr<sdk::MValueMutWrapper>);

macro_rules! impl_serializable {
    (@internal $value_type: ty, $create_mvalue: expr) => {
        impl TryFrom<$value_type> for Serializable {
            type Error = anyhow::Error;
            fn try_from(value: $value_type) -> SomeResult<Self> {
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
impl_serializable!(&str, sdk::create_mvalue_string);
impl_serializable!(String, sdk::create_mvalue_string);

impl_serializable!(i8, |value| sdk::create_mvalue_int(value as i64));
impl_serializable!(i16, |value| sdk::create_mvalue_int(value as i64));
impl_serializable!(i32, |value| sdk::create_mvalue_int(value as i64));
impl_serializable!(i64, sdk::create_mvalue_int);

impl_serializable!(f32, |value| sdk::create_mvalue_double(value as f64));
impl_serializable!(f64, sdk::create_mvalue_double);

impl_serializable!(u8, |value| sdk::create_mvalue_uint(value as u64));
impl_serializable!(u16, |value| sdk::create_mvalue_uint(value as u64));
impl_serializable!(u32, |value| sdk::create_mvalue_uint(value as u64));
impl_serializable!(u64, sdk::create_mvalue_uint);

impl_serializable!(Vec<Serializable>, |value| sdk::create_mvalue_list(
    convert_iter_to_mvalue_vec(value)
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

            fn try_from(base_object: $base_object) -> SomeResult<Self> {
                let Ok(ptr) = base_object.base_ptr() else {
                                    anyhow::bail!("{} base object is destroyed", $short_name);
                                };
                Ok(Self(
                    unsafe { sdk::create_mvalue_base_object(ptr.as_ptr()) }.within_unique_ptr(),
                ))
            }
        }
    };
}

impl_serializable_base_object!(vehicle::VehicleContainer, "vehicle");
impl_serializable_base_object!(player::PlayerContainer, "player");
impl_serializable_base_object!(col_shape::ColShapeContainer, "colshape");
impl_serializable_base_object!(blip::BlipContainer, "blip");
impl_serializable_base_object!(marker::MarkerContainer, "marker");
impl_serializable_base_object!(checkpoint::CheckpointContainer, "checkpoint");
impl_serializable_base_object!(ped::PedContainer, "ped");
impl_serializable_base_object!(network_object::NetworkObjectContainer, "network object");

// TODO: fix this none/null/nil shit
/// alias for `MValue::None`
#[derive(Debug)]
pub struct MValueNone;

impl TryFrom<MValueNone> for Serializable {
    type Error = anyhow::Error;
    fn try_from(_: MValueNone) -> SomeResult<Self> {
        Ok(Self(
            unsafe { sdk::create_mvalue_nil() }.within_unique_ptr(),
        ))
    }
}

pub fn convert_iter_to_mvalue_vec(
    iter: impl IntoIterator<Item = Serializable>,
) -> UniquePtr<CxxVector<sdk::MValueWrapper>> {
    let mut mvalue_vec = unsafe { sdk::create_mvalue_vec() };

    for value in iter {
        unsafe {
            sdk::push_to_mvalue_vec(mvalue_vec.as_mut().unwrap(), value.0);
        }
    }

    mvalue_vec
}

pub fn convert_player_vec_to_cpp_vec(
    vec: Vec<player::PlayerContainer>,
) -> SomeResult<UniquePtr<CxxVector<sdk::PlayerPtrWrapper>>> {
    let mut cpp_vec = unsafe { sdk::create_player_vec() };

    for player in vec {
        unsafe {
            sdk::push_to_player_vec(cpp_vec.as_mut().unwrap(), player.raw_ptr()?);
        }
    }

    Ok(cpp_vec)
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

    ColShape(col_shape::ColShapeContainer),
    Vehicle(vehicle::VehicleContainer),
    Player(player::PlayerContainer),
    Ped(ped::PedContainer),
    NetworkObject(network_object::NetworkObjectContainer),
    VirtualEntity(virtual_entity::VirtualEntityContainer),
    VirtualEntityGroup(virtual_entity_group::VirtualEntityGroupContainer),
    Blip(blip::BlipContainer),
    VoiceChannel(voice_channel::VoiceChannelContainer),
    Marker(marker::MarkerContainer),
    Checkpoint(checkpoint::CheckpointContainer),
    InvalidBaseObject,
}

macro_rules! get_mvalue_type_at {
    ($method_name: ident, $type_name: ty, $mvalue_type: path) => {
        pub fn $method_name(&self, index: usize) -> SomeResult<&$type_name> {
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

    pub fn get(&self, index: usize) -> SomeResult<&MValue> {
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
    get_mvalue_type_at!(get_player_at, player::PlayerContainer, MValue::Player);
    get_mvalue_type_at!(get_vehicle_at, vehicle::VehicleContainer, MValue::Vehicle);
    get_mvalue_type_at!(
        get_col_shape_at,
        col_shape::ColShapeContainer,
        MValue::ColShape
    );
    get_mvalue_type_at!(
        get_virtual_entity_at,
        virtual_entity::VirtualEntityContainer,
        MValue::VirtualEntity
    );
    get_mvalue_type_at!(
        get_virtual_entity_group_at,
        virtual_entity_group::VirtualEntityGroupContainer,
        MValue::VirtualEntityGroup
    );

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
    let mvalue_type = altv_sdk::MValueType::try_from(mvalue_type).unwrap();

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
            logger::debug!("deserializing baseobject raw ptr: {ptr:?}");

            let Some(ptr) = NonNull::new(ptr) else {
                return MValue::InvalidBaseObject;
            };

            let base_obj = resource.base_objects.borrow().get_by_ptr(ptr);
            let Some(base_obj) = base_obj else {
                let base_type = unsafe { get_base_object_type(ptr.as_ptr()) };
                logger::error!("[deserialize_mvalue] baseobject pointer is not null, but baseobject is not in pool (probably type is unknown? {base_type:?})");
                return MValue::InvalidBaseObject;
            };

            match base_obj {
                AnyBaseObject::ColShape(c) => MValue::ColShape(c),
                AnyBaseObject::Vehicle(c) => MValue::Vehicle(c),
                AnyBaseObject::Player(c) => MValue::Player(c),
                AnyBaseObject::VirtualEntity(c) => MValue::VirtualEntity(c),
                AnyBaseObject::VirtualEntityGroup(c) => MValue::VirtualEntityGroup(c),
                AnyBaseObject::Blip(c) => MValue::Blip(c),
                AnyBaseObject::VoiceChannel(c) => MValue::VoiceChannel(c),
                AnyBaseObject::Marker(c) => MValue::Marker(c),
                AnyBaseObject::Checkpoint(c) => MValue::Checkpoint(c),
                AnyBaseObject::Ped(c) => MValue::Ped(c),
                AnyBaseObject::NetworkObject(c) => MValue::NetworkObject(c),
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
macro_rules! __serialize_mvalue {
    ($value: expr, $vec: expr) => {
        let serializable = $crate::exports::mvalue::__internal::Serializable::try_from($value);

        match serializable {
            Ok(serialized) => {
                $vec.push(serialized);
            }
            Err(error) => {
                $crate::exports::anyhow::bail!(
                    "Failed to convert value: {} to mvalue, error: {}",
                    stringify!($value),
                    error
                );
            }
        }
    };
}

pub use __serialize_mvalue as serialize_mvalue;

pub(crate) fn deserialize_from_sdk(
    value: impl autocxx::moveit::New<Output = sdk::MValueWrapper>,
    resource: &Resource,
) -> MValue {
    deserialize_mvalue(value.within_unique_ptr().as_ref().unwrap(), resource)
}
