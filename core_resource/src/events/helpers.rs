use std::ptr::NonNull;

use crate::{
    base_objects::{connection_info, vehicle, AnyBaseObject},
    resource::Resource,
    sdk,
};

#[macro_export]
macro_rules! __base_event_to_specific {
    ($base_event:expr, $specific:ident) => {
        paste::paste! {{
            let raw = $crate::sdk::events::[<to_ $specific>]($base_event);
            assert!(!raw.is_null(), "converting base event to: {} returned null", stringify!($specific));
            raw
        }}
    };
}
pub use __base_event_to_specific as base_event_to_specific;

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

pub fn get_connection_info_from_event(
    ptr: *mut sdk::alt::IConnectionInfo,
    resource: &Resource,
) -> connection_info::ConnectionInfoContainer {
    let ptr = NonNull::new(ptr).unwrap();

    resource
        .base_objects
        .borrow()
        .connection_info
        .get_by_ptr(ptr)
        .unwrap()
}
