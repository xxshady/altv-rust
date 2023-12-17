use altv_wasm_shared::{BaseObjectType, BaseObjectPtr};

use crate::helpers::{get_local_vehicle_by_ptr, get_vehicle_by_ptr};
use super::objects::{vehicle::Vehicle, local_vehicle::LocalVehicle};

#[derive(Debug)]
pub enum AnyEntity {
    Vehicle(Vehicle),
    LocalVehicle(LocalVehicle),
}

pub(crate) fn get_any_entity_by_ptr_ty(ptr: BaseObjectPtr, ty: BaseObjectType) -> AnyEntity {
    match ty {
        BaseObjectType::Vehicle => AnyEntity::Vehicle(get_vehicle_by_ptr(ptr)),
        BaseObjectType::LocalVehicle => AnyEntity::LocalVehicle(get_local_vehicle_by_ptr(ptr)),
        _ => panic!("unknown vehicle type: {ty:?}"),
    }
}
