use crate::State;
use super::{objects::vehicle::Vehicle, remote::RemoteBaseObject, shared_vehicle::SharedVehicle};

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

impl Vehicle {}

impl RemoteBaseObject for Vehicle {}
impl SharedVehicle for Vehicle {}
