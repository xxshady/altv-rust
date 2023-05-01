use crate::{
    base_objects::AnyBaseObject,
    helpers::{self, IntoString},
    meta, sdk,
    vector::Vector3,
};

pub fn get_closest_entities(
    pos: impl Into<Vector3>,
    range: i32,
    dimension: i32,
    limit: i32,
    allowed_types: u64,
) -> Vec<AnyBaseObject> {
    let pos = pos.into();
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
    helpers::read_cpp_base_object_vec(raw)
}

pub fn get_entities_in_dimension(dimension: i32, allowed_types: u64) -> Vec<AnyBaseObject> {
    let raw = unsafe { sdk::ICore::GetEntitiesInDimension(dimension, allowed_types) };
    helpers::read_cpp_base_object_vec(raw)
}

pub fn get_entities_in_range(
    pos: impl Into<Vector3>,
    range: i32,
    dimension: i32,
    allowed_types: u64,
) -> Vec<AnyBaseObject> {
    let pos = pos.into();
    let raw = unsafe {
        sdk::ICore::GetEntitiesInRange(pos.x(), pos.y(), pos.z(), range, dimension, allowed_types)
    };
    helpers::read_cpp_base_object_vec(raw)
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

pub fn toggle_world_profiler(toggle: bool) {
    unsafe { sdk::ICore::SetWorldProfiler(toggle) }
}

pub fn get_net_time() -> u32 {
    unsafe { sdk::ICore::GetNetTime() }
}

meta::impl_meta!(Meta, sdk::ICore);
meta::impl_meta!(SyncedMeta, sdk::ICore);
