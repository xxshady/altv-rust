use altv_wasm_shared::{BaseObjectPtr, BaseObjectType};

use super::{local_vehicle::LOCAL_VEHICLE_STORE, vehicle::VEHICLE_STORE};

#[derive(Debug, Default)]
pub(crate) struct BaseObjectManager {}

impl BaseObjectManager {
    pub(crate) fn on_create(&mut self, ptr: BaseObjectPtr, ty: BaseObjectType) {
        logger::debug!("on_create base object ty: {ty:?}");

        match ty {
            BaseObjectType::LocalVehicle => {
                LOCAL_VEHICLE_STORE.with(|v| {
                    v.borrow_mut().insert(ptr);
                });
            }
            BaseObjectType::Vehicle => {
                VEHICLE_STORE.with(|v| {
                    v.borrow_mut().insert(ptr);
                });
            }
            _ => {
                // TODO: add logger for internal stuff of wasm
                logger::error!("Create unknown base object: {ptr:?} {ty:?}");
            }
        }
    }

    pub(crate) fn on_destroy(&mut self, ptr: BaseObjectPtr, ty: BaseObjectType) {
        logger::debug!("on_destroy base object ty: {ty:?}");

        match ty {
            BaseObjectType::LocalVehicle => {
                LOCAL_VEHICLE_STORE.with(|v| {
                    v.borrow_mut().remove(&ptr);
                });
            }
            BaseObjectType::Vehicle => {
                VEHICLE_STORE.with(|v| {
                    v.borrow_mut().remove(&ptr);
                });
            }
            _ => {
                // TODO: add logger for internal stuff of wasm
                logger::error!("Create unknown base object: {ptr:?} {ty:?}");
            }
        }
    }
}
