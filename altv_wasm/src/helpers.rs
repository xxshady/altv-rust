use crate::{
    LocalVehicle, Vehicle,
    state::State,
    base_objects::{objects::ObjectData, kind::BaseObjectKind},
};
use altv_wasm_shared::BaseObjectPtr;

pub(crate) fn get_vehicle_by_ptr(ptr: BaseObjectPtr) -> Vehicle {
    match Vehicle::new(ptr) {
        Some(v) => v,
        // vehicle must be spawned
        None => unreachable!(),
    }
}

pub(crate) fn get_local_vehicle_by_ptr(ptr: BaseObjectPtr) -> LocalVehicle {
    State::with_base_objects_mut(|mut objects, _| match objects.all.get(&ptr) {
        None => {
            panic!("Unknown local vehicle ptr: {ptr}");
        }
        Some(ObjectData {
            kind: BaseObjectKind::LocalVehicleUnknown,
            ..
        }) => {
            // if instance is created by other resource pass it as borrowed
            LocalVehicle::internal_new_borrowed(ptr, ())
        }
        Some(_) => LocalVehicle::internal_new_owned(ptr, &mut objects.all, ()),
    })
}
