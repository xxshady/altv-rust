use std::ptr::NonNull;

use crate::{
    base_objects::{extra_pools::EntityRawPtr, player, vehicle, AnyBaseObject},
    exports::AnyEntity,
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

pub fn get_player_from_event<T>(
    event: *const T,
    resource: &Resource,
    get_target: unsafe fn(*const T) -> *mut sdk::alt::IPlayer,
) -> player::PlayerContainer {
    let ptr = unsafe { get_target(event) };
    let ptr = NonNull::new(ptr).unwrap();

    resource
        .base_objects
        .borrow()
        .player
        .get_by_ptr(ptr)
        .unwrap()
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

pub fn get_non_null_entity_from_event<T>(
    event: *const T,
    resource: &Resource,
    get_target: unsafe fn(*const T) -> EntityRawPtr,
) -> AnyEntity {
    let entity = unsafe { get_target(event) };
    let entity = NonNull::new(entity).unwrap().as_ptr();
    get_entity_from_event(entity, resource).unwrap()
}

pub fn get_entity_from_event(entity: EntityRawPtr, resource: &Resource) -> Option<AnyEntity> {
    if entity.is_null() {
        return None;
    }
    let entity = unsafe { sdk::entity::to_base_object(entity) };
    let entity = NonNull::new(entity).unwrap();
    let entity = resource.base_objects.borrow().get_by_ptr(entity).unwrap();
    let entity = match entity {
        AnyBaseObject::Player(p) => AnyEntity::Player(p),
        AnyBaseObject::Vehicle(v) => AnyEntity::Vehicle(v),
        _ => {
            unreachable!()
        }
    };
    Some(entity)
}
