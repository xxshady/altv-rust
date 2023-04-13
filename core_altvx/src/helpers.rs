use std::ptr::NonNull;

use crate::{
    base_objects::extra_pools::{wrappers::AnyEntity, EntityRawPtr},
    resource::Resource,
    structs,
    vector::{Vector2, Vector3},
};
use altv_sdk::ffi as sdk;
use autocxx::prelude::*;

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

pub fn read_rgba(cpp_rgba: UniquePtr<sdk::RGBAWrapper>) -> structs::RGBA {
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
use structs::RGBA;

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
