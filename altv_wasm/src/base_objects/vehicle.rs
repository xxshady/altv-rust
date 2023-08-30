use altv_wasm_shared::BaseObjectPtr;
use crate::{__imports, State};

#[derive(Debug, Default)]
pub struct VehicleManager {
    objects: Vec<Vehicle>,
}

impl VehicleManager {
    pub fn all(&mut self) -> &[Vehicle] {
        State::with_base_objects_ref(|base_objects, _| {
            self.objects = base_objects
                .vehicles()
                .map(|ptr| Vehicle { ptr: *ptr })
                .collect();
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

macro_rules! assert_vehicle_is_valid {
    ($object:ident) => {
        let valid = State::with_base_objects_ref(|base_objects, _| {
            base_objects.vehicles().any(|ptr| *ptr == $object.ptr)
        });
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
