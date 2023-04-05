use std::ptr::NonNull;

use crate::{
    base_object::{player, AnyBaseObject},
    resource::Resource,
    vector::{Vector2, Vector3},
    SomeResult,
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

// credits to altv-rs creator
// https://github.com/justdimaa/altv-rs/blob/f5cf1733493466634793804dfb1ca6d387fbe687/altv-sdk/src/lib.rs#L24
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

pub fn get_player_from_event<T>(
    event: *const T,
    resource: &Resource,
    get_target: unsafe fn(*const T) -> *mut sdk::alt::IPlayer,
) -> player::PlayerContainer {
    let ptr = unsafe { get_target(event) };
    let ptr = NonNull::new(unsafe { sdk::player::to_base_object(ptr) }).unwrap();

    let base_object = resource.base_objects.borrow().get_by_ptr(ptr).unwrap();
    if let AnyBaseObject::Player(p) = base_object {
        p
    } else {
        unreachable!()
    }
}

pub fn get_player_raw_ptr(player: player::PlayerContainer) -> SomeResult<*mut sdk::alt::IPlayer> {
    Ok(player.try_borrow()?.ptr()?.as_ptr())
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
