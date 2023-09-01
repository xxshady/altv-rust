use crate::{
    base_objects::AnyBaseObject,
    helpers::{self, IntoHash},
    sdk,
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

pub fn set_password(password: impl ToString) {
    unsafe { sdk::ICore::SetPassword(password.to_string()) }
}

pub fn hash_server_password(password: impl ToString) -> u64 {
    unsafe { sdk::ICore::HashServerPassword(password.to_string()) }
}

pub fn toggle_world_profiler(toggle: bool) {
    unsafe { sdk::ICore::SetWorldProfiler(toggle) }
}

pub fn net_time() -> u32 {
    unsafe { sdk::ICore::GetNetTime() }
}

pub fn get_ammo_hash_for_weapon_hash(weapon_hash: impl IntoHash) -> Option<u32> {
    let ammo_hash = unsafe { sdk::ICore::GetAmmoHashForWeaponHash(weapon_hash.into_hash()) };
    if ammo_hash != 0 {
        Some(ammo_hash)
    } else {
        None
    }
}

pub fn set_voice_external_public(host: impl ToString, port: u16) {
    unsafe { sdk::ICore::SetVoiceExternalPublic(host.to_string(), port) }
}

pub fn set_voice_external(host: impl ToString, port: u16) {
    unsafe { sdk::ICore::SetVoiceExternal(host.to_string(), port) }
}

pub fn get_voice_connection_state() -> altv_sdk::VoiceConnectionState {
    altv_sdk::VoiceConnectionState::try_from(unsafe { sdk::ICore::GetVoiceConnectionState() })
        .unwrap()
}

pub fn max_streaming_peds() -> u16 {
    unsafe { sdk::ICore::GetMaxStreamingPeds() }
}

pub fn set_max_streaming_peds(value: u16) {
    unsafe { sdk::ICore::SetMaxStreamingPeds(value) }
}

pub fn max_streaming_objects() -> u16 {
    unsafe { sdk::ICore::GetMaxStreamingObjects() }
}

pub fn set_max_streaming_objects(value: u16) {
    unsafe { sdk::ICore::SetMaxStreamingObjects(value) }
}

pub fn max_streaming_vehicles() -> u16 {
    unsafe { sdk::ICore::GetMaxStreamingVehicles() }
}

pub fn set_max_streaming_vehicles(value: u16) {
    unsafe { sdk::ICore::SetMaxStreamingVehicles(value) }
}

pub fn streamer_thread_count() -> u8 {
    unsafe { sdk::ICore::GetStreamerThreadCount() }
}

pub fn set_streamer_thread_count(value: u8) {
    unsafe { sdk::ICore::SetStreamerThreadCount(value) }
}

pub fn migration_thread_count() -> u8 {
    unsafe { sdk::ICore::GetMigrationThreadCount() }
}

pub fn set_migration_thread_count(value: u8) {
    unsafe { sdk::ICore::SetMigrationThreadCount(value) }
}

pub fn sync_send_thread_count() -> u8 {
    unsafe { sdk::ICore::GetSyncSendThreadCount() }
}

pub fn set_sync_send_thread_count(value: u8) {
    unsafe { sdk::ICore::SetSyncSendThreadCount(value) }
}

pub fn sync_receive_thread_count() -> u8 {
    unsafe { sdk::ICore::GetSyncReceiveThreadCount() }
}

pub fn set_sync_receive_thread_count(value: u8) {
    unsafe { sdk::ICore::SetSyncReceiveThreadCount(value) }
}

pub fn streaming_tick_rate() -> u32 {
    unsafe { sdk::ICore::GetStreamingTickRate() }
}

pub fn set_streaming_tick_rate(value: u32) {
    unsafe { sdk::ICore::SetStreamingTickRate(value) }
}

pub fn migration_tick_rate() -> u32 {
    unsafe { sdk::ICore::GetMigrationTickRate() }
}

pub fn set_migration_tick_rate(value: u32) {
    unsafe { sdk::ICore::SetMigrationTickRate(value) }
}

pub fn col_shape_tick_rate() -> u32 {
    unsafe { sdk::ICore::GetColShapeTickRate() }
}

pub fn set_col_shape_tick_rate(value: u32) {
    unsafe { sdk::ICore::SetColShapeTickRate(value) }
}

pub fn streaming_distance() -> u32 {
    unsafe { sdk::ICore::GetStreamingDistance() }
}

pub fn set_streaming_distance(value: u32) {
    unsafe { sdk::ICore::SetStreamingDistance(value) }
}

pub fn migration_distance() -> u32 {
    unsafe { sdk::ICore::GetMigrationDistance() }
}

pub fn set_migration_distance(value: u32) {
    unsafe { sdk::ICore::SetMigrationDistance(value) }
}
