use autocxx::{
    cxx::{CxxString, CxxVector},
    prelude::*,
};
use lazycell::LazyCell;
use std::{fmt::Debug, ptr::NonNull};

use crate::{
    base_objects::{
        extra_pools::{
            wrappers::{AnyEntity, AnyWorldObject},
            EntityRawPtr,
        },
        player, AnyBaseObject,
    },
    quaternion::Quaternion,
    resource::Resource,
    rgba::RGBA,
    vector::{Vector2, Vector3},
    world_object::WorldObjectRawPtr,
    SomeResult,
};
use altv_sdk::ffi as sdk;

pub fn read_cpp_vector3(cpp_vector: UniquePtr<sdk::Vector3Wrapper>) -> Vector3 {
    let mut out_x = 0f32;
    let mut out_y = 0f32;
    let mut out_z = 0f32;
    unsafe {
        sdk::read_vector3(
            cpp_vector.as_ref().unwrap(),
            &mut out_x as *mut f32,
            &mut out_y as *mut f32,
            &mut out_z as *mut f32,
        );
    }
    Vector3::new(out_x, out_y, out_z)
}

pub fn read_cpp_vector2(cpp_vector: UniquePtr<sdk::Vector2Wrapper>) -> Vector2 {
    let mut out_x = 0f32;
    let mut out_y = 0f32;
    unsafe {
        sdk::read_vector2(
            cpp_vector.as_ref().unwrap(),
            &mut out_x as *mut f32,
            &mut out_y as *mut f32,
        );
    }
    Vector2::new(out_x, out_y)
}

pub fn read_cpp_rgba(cpp_rgba: UniquePtr<sdk::RGBAWrapper>) -> RGBA {
    let mut r = 0u8;
    let mut g = 0u8;
    let mut b = 0u8;
    let mut a = 0u8;
    unsafe {
        sdk::read_rgba(
            cpp_rgba.as_ref().unwrap(),
            &mut r as *mut _,
            &mut g as *mut _,
            &mut b as *mut _,
            &mut a as *mut _,
        );
    }

    RGBA::new(r, g, b, a)
}

pub fn read_cpp_quaternion(cpp_quaternion: UniquePtr<sdk::alt::Quaternion>) -> Quaternion {
    let mut out_x = 0f32;
    let mut out_y = 0f32;
    let mut out_z = 0f32;
    let mut out_w = 0f32;
    unsafe {
        sdk::read_quaternion(
            cpp_quaternion.as_ref().unwrap(),
            &mut out_x as *mut f32,
            &mut out_y as *mut f32,
            &mut out_z as *mut f32,
            &mut out_w as *mut f32,
        );
    }
    Quaternion::new(out_x, out_y, out_z, out_w)
}

// credits to altv-rs creator
// https://github.com/justdimaa/altv-rs/blob/f5cf1733493466634793804dfb1ca6d387fbe687/altv-sdk/src/lib.rs#L24
/// joaat hash function
pub fn hash(str: &str) -> u32 {
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

pub trait IntoString {
    fn into_string(self) -> String;
}

impl IntoString for String {
    fn into_string(self) -> String {
        self
    }
}

impl IntoString for &str {
    fn into_string(self) -> String {
        self.to_string()
    }
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

#[macro_export]
macro_rules! __get_any_option_base_object {
    ($get_ptr: expr, $base_obj_manager: ident) => {
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

#[macro_export]
macro_rules! __if_not {
    (() $code: block) => {
        $code
    };
    (($( $target: tt )+) $code: block) => {
        $( $target )+
    };
}
pub use __if_not as if_not;

pub fn init_or_get_lazycell<T: Debug>(
    cell: &LazyCell<T>,
    init: impl FnOnce() -> SomeResult<T>,
) -> SomeResult<&T> {
    if cell.filled() {
        logger::debug!("lazycell filled");
        return Ok(cell.borrow().unwrap());
    }
    logger::debug!("lazycell is not filled");

    cell.fill(init()?).unwrap();
    Ok(cell.borrow().unwrap())
}

#[macro_export]
macro_rules! __base_ptr_to {
    ($base_ptr: expr, $target_type: ident) => {
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
    ($base_ptr: expr, $target_type: ident) => {
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
