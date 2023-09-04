use crate::{__imports, State, base_objects::objects::vehicle::Vehicle};

#[derive(Debug, Default)]
pub struct VehicleManager {
    objects: Vec<Vehicle>,
}

impl VehicleManager {
    pub fn all(&mut self) -> &[Vehicle] {
        State::with_base_objects_ref(|base_objects, _| {
            self.objects = base_objects
                .vehicle_iter()
                .map(|ptr| Vehicle::new(*ptr))
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

impl Vehicle {
    // TODO: move it to RemoteBaseObject trait?
    pub fn remote_id(&self) -> u32 {
        __imports::base_object_get_remote_id(self.ptr())
    }

    pub fn fuel_level(&self) -> f32 {
        __imports::vehicle_get_fuel_level(self.ptr())
    }

    pub fn set_fuel_level(&self, value: f32) {
        __imports::vehicle_set_fuel_level(self.ptr(), value);
    }
}
