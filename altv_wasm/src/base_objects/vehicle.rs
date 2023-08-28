use std::{cell::RefCell, collections::HashSet};

use altv_wasm_shared::BaseObjectPtr;
use crate::__imports;

#[derive(Debug, Default)]
pub struct VehicleManager {
    objects: Vec<Vehicle>,
}

impl VehicleManager {
    pub fn all(&mut self) -> &[Vehicle] {
        VEHICLE_STORE.with(|v| {
            self.objects = v.borrow().iter().map(|ptr| Vehicle { ptr: *ptr }).collect();
        });

        &self.objects
    }

    pub fn get_by_id(&mut self, id: u32) -> Option<&Vehicle> {
        self.all().iter().find(|v| v.id() == id)
    }

    pub fn get_by_remote_id(&mut self, id: u32) -> Option<&Vehicle> {
        self.all().iter().find(|v| v.remote_id() == id)
    }
}

thread_local! {
    pub static VEHICLE_STORE: RefCell<HashSet<BaseObjectPtr>> = RefCell::new(HashSet::new());
}

macro_rules! assert_vehicle_is_valid {
    ($object:ident) => {
        let valid =
            VEHICLE_STORE.with(|store| store.borrow().iter().any(|ptr| *ptr == $object.ptr));
        assert!(valid, "Vehicle instance is invalid");
    };
}

#[derive(Debug)]
pub struct Vehicle {
    pub(crate) ptr: BaseObjectPtr,
}

impl Vehicle {
    pub fn id(&self) -> u32 {
        assert_vehicle_is_valid!(self);
        __imports::base_object_get_id(self.ptr)
    }

    pub fn remote_id(&self) -> u32 {
        assert_vehicle_is_valid!(self);
        __imports::base_object_get_remote_id(self.ptr)
    }

    pub fn fuel_level(&self) -> f32 {
        assert_vehicle_is_valid!(self);
        __imports::vehicle_get_fuel_level(self.ptr)
    }

    pub fn set_fuel_level(&self, value: f32) {
        assert_vehicle_is_valid!(self);
        __imports::vehicle_set_fuel_level(self.ptr, value);
    }
}
