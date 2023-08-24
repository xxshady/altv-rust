use altv_wasm_shared::{BaseObjectPtr, BaseObjectType};

use crate::log_error;

use super::local_vehicle::LOCAL_VEHICLE_STORE;

#[derive(Debug, Default)]
pub(crate) struct BaseObjectManager {}

impl BaseObjectManager {
    pub(crate) fn on_create(&mut self, ptr: BaseObjectPtr, ty: BaseObjectType) {
        match ty {
            BaseObjectType::LocalVehicle => {
                LOCAL_VEHICLE_STORE.with(|v| {
                    v.borrow_mut().insert(ptr);
                });
            }
            _ => {
                // TODO: add logger for internal stuff of wasm
                log_error!("Create unknown base object: {ptr:?} {ty:?}");
            }
        }
    }

    pub(crate) fn on_destroy(&mut self, ptr: BaseObjectPtr, ty: BaseObjectType) {
        match ty {
            BaseObjectType::LocalVehicle => {
                LOCAL_VEHICLE_STORE.with(|v| {
                    v.borrow_mut().remove(&ptr);
                });
            }
            _ => {
                // TODO: add logger for internal stuff of wasm
                log_error!("Create unknown base object: {ptr:?} {ty:?}");
            }
        }
    }
}
