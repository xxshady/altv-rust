use autocxx::{
    cxx::{CxxString, CxxVector},
    prelude::*,
};
use std::ptr::NonNull;

use crate::{
    base_objects::{
        extra_pools::{AnyEntity, AnyWorldObject, EntityRawPtr, WorldObjectRawPtr},
        player, AnyBaseObject, ped,
    },
    quaternion::Quaternion,
    resource::Resource,
    rgba::Rgba,
    structs,
    vector::{Vector2, Vector3},
    SomeResult,
};
use altv_sdk::ffi as sdk;

pub fn read_cpp_vector3(cpp_vector: UniquePtr<sdk::Vector3Wrapper>) -> Vector3 {
    let (mut x, mut y, mut z) = Default::default();
    unsafe {
        sdk::read_vector3(cpp_vector.as_ref().unwrap(), &mut x, &mut y, &mut z);
    }
    Vector3::new(x, y, z)
}

pub fn read_cpp_vector2(cpp_vector: UniquePtr<sdk::Vector2Wrapper>) -> Vector2 {
    let (mut x, mut y) = Default::default();
    unsafe {
        sdk::read_vector2(cpp_vector.as_ref().unwrap(), &mut x, &mut y);
    }
    Vector2::new(x, y)
}

pub fn read_cpp_rgba(cpp_rgba: UniquePtr<sdk::RGBAWrapper>) -> Rgba {
    let (mut r, mut g, mut b, mut a) = Default::default();
    unsafe {
        sdk::read_rgba(cpp_rgba.as_ref().unwrap(), &mut r, &mut g, &mut b, &mut a);
    }

    Rgba::new(r, g, b, a)
}

pub fn read_cpp_quaternion(cpp_quaternion: UniquePtr<sdk::alt::Quaternion>) -> Quaternion {
    let (mut x, mut y, mut z, mut w) = Default::default();
    unsafe {
        sdk::read_quaternion(
            cpp_quaternion.as_ref().unwrap(),
            &mut x,
            &mut y,
            &mut z,
            &mut w,
        );
    }
    Quaternion::new(x, y, z, w)
}

// credits to altv-rs creator
// https://github.com/justdimaa/altv-rs/blob/f5cf1733493466634793804dfb1ca6d387fbe687/altv-sdk/src/lib.rs#L24
/// joaat hash function
pub fn hash(str: &str) -> u32 {
    let str = str.to_lowercase();
    let bytes = str.as_bytes();
    let mut num: std::num::Wrapping<u32> = std::num::Wrapping(0u32);

    for n in bytes {
        num += std::num::Wrapping(*n as u32);
        num += num << 10;
        num ^= num >> 6;
    }

    num += num << 3;
    num ^= num >> 11;

    (num + (num << 15)).0
}

pub type Hash = u32;

pub trait IntoHash {
    fn into_hash(self) -> Hash;
}

impl IntoHash for Hash {
    fn into_hash(self) -> Hash {
        self
    }
}

impl IntoHash for &str {
    fn into_hash(self) -> Hash {
        hash(self)
    }
}

impl IntoHash for String {
    fn into_hash(self) -> Hash {
        hash(&self)
    }
}

impl IntoHash for structs::AmmoType {
    fn into_hash(self) -> Hash {
        self as Hash
    }
}

#[macro_export]
macro_rules! __get_any_option_base_object {
    ($get_ptr:expr, $base_obj_manager:ident) => {
        paste::paste! { {
            let ptr = unsafe { $get_ptr };
            let Some(ptr) = std::ptr::NonNull::new(ptr) else {
                return Ok(None);
            };
            Ok($crate::resource::Resource::with_base_objects_ref(|v, _| {
                v.[<$base_obj_manager>].get_by_ptr(ptr)
            }))
        } }
    };
}
pub(crate) use __get_any_option_base_object as get_any_option_base_object;

pub(crate) fn get_non_null_entity_by_ptr(ptr: EntityRawPtr, resource: &Resource) -> AnyEntity {
    let entity = NonNull::new(ptr).unwrap().as_ptr();
    get_entity_by_ptr(entity, resource).unwrap()
}

pub(crate) fn get_entity_by_ptr(entity: EntityRawPtr, resource: &Resource) -> Option<AnyEntity> {
    if entity.is_null() {
        return None;
    }
    let entity = unsafe { sdk::entity::to_base_object(entity) };
    let entity = NonNull::new(entity).unwrap();
    let base_object = resource.base_objects.borrow().get_by_ptr(entity).unwrap();
    let entity = base_object.try_into().unwrap();
    Some(entity)
}

pub(crate) fn get_non_null_world_object_by_ptr(
    ptr: WorldObjectRawPtr,
    resource: &Resource,
) -> AnyWorldObject {
    let world_object = NonNull::new(ptr).unwrap().as_ptr();
    get_world_object_by_ptr(world_object, resource).unwrap()
}

pub(crate) fn get_world_object_by_ptr(
    world_object: WorldObjectRawPtr,
    resource: &Resource,
) -> Option<AnyWorldObject> {
    if world_object.is_null() {
        return None;
    }
    let world_object = unsafe { sdk::world_object::to_base_object(world_object) };
    let world_object = NonNull::new(world_object).unwrap();
    let base_object = resource
        .base_objects
        .borrow()
        .get_by_ptr(world_object)
        .unwrap();
    let world_object = base_object.try_into().unwrap();
    Some(world_object)
}

pub trait IntoF32: Copy {
    fn into_f32(self) -> f32;
}

impl IntoF32 for f32 {
    fn into_f32(self) -> f32 {
        self
    }
}

impl IntoF32 for i32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

pub(crate) fn read_cpp_str_vec(cpp_vec: UniquePtr<CxxVector<CxxString>>) -> Vec<String> {
    cpp_vec.into_iter().map(|v| v.to_string()).collect()
}

pub fn get_non_null_player(
    ptr: *mut sdk::alt::IPlayer,
    resource: &Resource,
) -> player::PlayerContainer {
    get_player(ptr, resource).unwrap()
}

pub fn get_player(
    ptr: *mut sdk::alt::IPlayer,
    resource: &Resource,
) -> Option<player::PlayerContainer> {
    let Some(ptr) = NonNull::new(ptr) else {
        return None;
    };

    let player = resource
        .base_objects
        .borrow()
        .player
        .get_by_ptr(ptr)
        .unwrap();
    Some(player)
}

pub fn get_non_null_ped(ptr: *mut sdk::alt::IPed, resource: &Resource) -> ped::PedContainer {
    get_ped(ptr, resource).unwrap()
}

pub fn get_ped(ptr: *mut sdk::alt::IPed, resource: &Resource) -> Option<ped::PedContainer> {
    let Some(ptr) = NonNull::new(ptr) else {
        return None;
    };

    let ped = resource.base_objects.borrow().ped.get_by_ptr(ptr).unwrap();
    Some(ped)
}

#[macro_export]
macro_rules! __if_not {
    (() $code:block) => {
        $code
    };
    (($( $target:tt )+) $code:block) => {
        $( $target )+
    };
}
pub use __if_not as if_not;

#[macro_export]
macro_rules! __base_ptr_to {
    ($base_ptr:expr, $target_type:ident) => {
        paste::paste! {
            unsafe {
                std::ptr::NonNull::new($crate::sdk::base_object::[<to_ $target_type>]($base_ptr)).unwrap()
            }
        }
    };
}

pub use __base_ptr_to as base_ptr_to;

#[macro_export]
macro_rules! __base_ptr_to_raw {
    ($base_ptr:expr, $target_type:ident) => {
        $crate::helpers::base_ptr_to!($base_ptr, $target_type).as_ptr()
    };
}

pub use __base_ptr_to_raw as base_ptr_to_raw;

pub fn read_cpp_base_object_vec(
    cpp_vec: UniquePtr<CxxVector<sdk::BaseObjectPtrWrapper>>,
) -> Vec<AnyBaseObject> {
    cpp_vec
        .into_iter()
        .filter_map(|v| {
            let ptr = unsafe { sdk::read_base_object_ptr_wrapper(v) };
            let ptr = NonNull::new(ptr).unwrap();
            Resource::with_base_objects_ref(|v, _| v.get_by_ptr(ptr))
        })
        .collect()
}

macro_rules! __create_base_object {
    ($namespace:path, $creation:expr, $else:expr) => {{
        paste::paste! {
            let ptr = $crate::resource::Resource::with_pending_base_object_destroy_or_creation_mut(
                |_, _| unsafe { $creation },
            );

            let Some(ptr) = std::ptr::NonNull::new(ptr) else {
                $else
            };

            $namespace::add_to_pool!(ptr)
        }
    }};
}

pub(crate) use __create_base_object as create_base_object;

pub(crate) fn convert_player_slice_to_cpp_vec(
    players: &[player::PlayerContainer],
) -> SomeResult<UniquePtr<CxxVector<sdk::PlayerPtrWrapper>>> {
    let mut cpp_vec = unsafe { sdk::create_player_vec() };

    for player in players {
        unsafe {
            sdk::push_to_player_vec(cpp_vec.as_mut().unwrap(), player.raw_ptr()?);
        }
    }

    Ok(cpp_vec)
}
