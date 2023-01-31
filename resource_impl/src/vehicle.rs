use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

type EntityId = u16;

#[derive(Debug)]
pub struct Vehicle {
    id: EntityId,
}

impl Vehicle {
    pub fn id(&self) -> EntityId {
        self.id
    }
}

pub type VehicleContainer = Arc<Mutex<Option<Vehicle>>>;

#[derive(Debug)]
pub struct VehicleManager {
    // TODO: maybe use pointer instead of id?
    vehicles: HashMap<EntityId, VehicleContainer>,
}

impl VehicleManager {
    pub fn new() -> Self {
        VehicleManager {
            vehicles: HashMap::new(),
        }
    }

    pub fn create_vehicle(
        &mut self,
        model: u32,
        x: f32,
        y: f32,
        z: f32,
        rx: f32,
        ry: f32,
        rz: f32,
    ) -> Option<VehicleContainer> {
        let ptr = unsafe { altv_sdk::ffi::create_vehicle(model, x, y, z, rx, ry, rz) };
        if ptr.is_null() {
            return None;
        }

        let id = get_vehicle_id(ptr);
        let vehicle: VehicleContainer = Arc::new(Mutex::new(Some(Vehicle { id })));

        self.vehicles.insert(id, vehicle.clone());

        Some(vehicle)
    }

    // TODO:
    // pub fn destroy_vehicle(&mut self, vehicle: Vehicle) {
    //     self.vehicles.remove(id);
    // }

    pub fn on_vehicle_create(&mut self, vehicle: *mut altv_sdk::ffi::IVehicle) {
        let id = get_vehicle_id(vehicle);

        // vehicle is created in this resource
        if self.vehicles.get(&get_vehicle_id(vehicle)).is_some() {
            // TODO: TEST
            dbg!("vehicle is created here: {id}");
            return;
        }

        // TODO: TEST
        dbg!("on_vehicle_create vehicle is created externally: {id}");

        let vehicle: VehicleContainer = Arc::new(Mutex::new(Some(Vehicle { id })));

        self.vehicles.insert(id, vehicle);
    }

    // TODO:
    pub fn on_vehicle_remove(&self, vehicle: *mut altv_sdk::ffi::IVehicle) {}
}

fn get_vehicle_id(ptr: *const altv_sdk::ffi::IVehicle) -> EntityId {
    unsafe {
        altv_sdk::ffi::get_entity_id(
            altv_sdk::ffi::convert_vehicle_to_entity(ptr) as *mut altv_sdk::ffi::IEntity
        )
    }
}
