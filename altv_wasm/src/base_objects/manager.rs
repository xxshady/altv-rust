use std::collections::{HashSet, hash_set};

use altv_wasm_shared::{BaseObjectPtr, BaseObjectType};

#[derive(Debug, Default)]
pub(crate) struct BaseObjectManager {
    vehicles: HashSet<BaseObjectPtr>,
    local_vehicles: HashSet<BaseObjectPtr>,
}

impl BaseObjectManager {
    pub(crate) fn vehicles(&self) -> hash_set::Iter<BaseObjectPtr> {
        self.vehicles.iter()
    }

    pub(crate) fn local_vehicles(&self) -> hash_set::Iter<BaseObjectPtr> {
        self.local_vehicles.iter()
    }

    pub(crate) fn on_create(&mut self, ptr: BaseObjectPtr, ty: BaseObjectType) {
        logger::debug!("on_create base object ty: {ty:?}");

        match ty {
            BaseObjectType::LocalVehicle => {
                self.local_vehicles.insert(ptr);
            }
            BaseObjectType::Vehicle => {
                self.vehicles.insert(ptr);
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
                self.local_vehicles.remove(&ptr);
            }
            BaseObjectType::Vehicle => {
                self.vehicles.remove(&ptr);
            }
            _ => {
                // TODO: add logger for internal stuff of wasm
                logger::error!("Create unknown base object: {ptr:?} {ty:?}");
            }
        }
    }
}
