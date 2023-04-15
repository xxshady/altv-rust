use std::ptr::NonNull;

use autocxx::{cxx::CxxVector, prelude::*};

use crate::{
    base_objects::AnyBaseObject, helpers::IntoString, resource::Resource, sdk, vector::IntoVector3,
};

pub fn get_closest_entities(
    pos: impl IntoVector3,
    range: i32,
    dimension: i32,
    limit: i32,
    allowed_types: u64,
) -> Vec<AnyBaseObject> {
    let pos = pos.into_vector3();
    let raw = unsafe {
        sdk::ICore::GetClosestEntities(
            pos.x(),
            pos.y(),
            pos.z(),
            range,
            dimension,
            limit,
            allowed_types,
        )
    };
    read_cpp_base_object_vec(raw)
}

pub fn get_entities_in_dimension(dimension: i32, allowed_types: u64) -> Vec<AnyBaseObject> {
    let raw = unsafe { sdk::ICore::GetEntitiesInDimension(dimension, allowed_types) };
    read_cpp_base_object_vec(raw)
}

pub fn get_entities_in_range(
    pos: impl IntoVector3,
    range: i32,
    dimension: i32,
    allowed_types: u64,
) -> Vec<AnyBaseObject> {
    let pos = pos.into_vector3();
    let raw = unsafe {
        sdk::ICore::GetEntitiesInRange(pos.x(), pos.y(), pos.z(), range, dimension, allowed_types)
    };
    read_cpp_base_object_vec(raw)
}

pub fn stop_server() {
    unsafe { sdk::ICore::StopServer() }
}

pub fn set_password(password: impl IntoString) {
    unsafe { sdk::ICore::SetPassword(password.into_string()) }
}

pub fn hash_server_password(password: impl IntoString) -> u64 {
    unsafe { sdk::ICore::HashServerPassword(password.into_string()) }
}

fn read_cpp_base_object_vec(
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
