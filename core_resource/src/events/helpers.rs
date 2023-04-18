use std::ptr::NonNull;

use crate::{
    base_objects::{player, vehicle, AnyBaseObject},
    resource::Resource,
    sdk,
};

#[macro_export]
macro_rules! __base_event_to_specific {
    ($base_event: expr, $specific: ident) => {
        paste::paste! {{
            let raw = $crate::sdk::events::[<to_ $specific>]($base_event);
            assert!(!raw.is_null(), "converting base event to: {} returned null", stringify!($specific));
            raw
        }}
    };
}
pub use __base_event_to_specific as base_event_to_specific;

pub fn get_non_null_player_from_event(
    ptr: *mut sdk::alt::IPlayer,
    resource: &Resource,
) -> player::PlayerContainer {
    get_player_from_event(ptr, resource).unwrap()
}

pub fn get_player_from_event(
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

pub fn get_vehicle_from_event(
    ptr: *mut sdk::alt::IVehicle,
    resource: &Resource,
) -> vehicle::VehicleContainer {
    let ptr = NonNull::new(ptr).unwrap();

    resource
        .base_objects
        .borrow()
        .vehicle
        .get_by_ptr(ptr)
        .unwrap()
}

pub fn get_non_null_base_object_from_event(
    ptr: altv_sdk::BaseObjectRawMutPtr,
    resource: &Resource,
) -> AnyBaseObject {
    get_base_object_from_event(ptr, resource).unwrap()
}

pub fn get_base_object_from_event(
    ptr: altv_sdk::BaseObjectRawMutPtr,
    resource: &Resource,
) -> Option<AnyBaseObject> {
    let Some(ptr) = NonNull::new(ptr) else {
        return None;
    };
    let base_object = resource.base_objects.borrow().get_by_ptr(ptr).unwrap();
    Some(base_object)
}
