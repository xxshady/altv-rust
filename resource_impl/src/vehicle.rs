use altv_sdk::ffi as sdk;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

type EntityId = u16;
type RawVehiclePointer = *mut sdk::IVehicle;

#[derive(Debug)]
struct VehiclePointer(RawVehiclePointer);

// TEST
unsafe impl Send for VehiclePointer {}
unsafe impl Sync for VehiclePointer {}

#[derive(Debug)]
pub struct Vehicle {
    id: EntityId,
    ptr: Option<VehiclePointer>,
}

impl Vehicle {
    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn set_secondary_color(&mut self, color: u8) {
        unsafe { sdk::set_vehicle_primary_color(self.ptr.as_ref().unwrap().0, color) }
    }

    pub fn get_secondary_color(&mut self) -> u8 {
        unsafe { sdk::get_vehicle_primary_color(self.ptr.as_ref().unwrap().0) }
    }

    pub fn destroy(&mut self) {
        crate::resource_impl::destroy_vehicle(self);
        self.ptr = None;
    }
}

pub type VehicleContainer = Arc<Mutex<Vehicle>>;

#[derive(Debug)]
pub struct PendingVehicleCreation {}

impl PendingVehicleCreation {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct PendingVehicleDeletion {}

impl PendingVehicleDeletion {
    pub fn new() -> Self {
        Self {}
    }
}

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
        mut pending_state: MutexGuard<PendingVehicleCreation>,
        model: u32,
        x: f32,
        y: f32,
        z: f32,
        rx: f32,
        ry: f32,
        rz: f32,
    ) -> Option<VehicleContainer> {
        let ptr = unsafe { sdk::create_vehicle(model, x, y, z, rx, ry, rz) };

        if ptr.is_null() {
            return None;
        }

        let id = get_vehicle_id(ptr);
        let vehicle: VehicleContainer = create_vehicle_container(id, ptr);

        self.vehicles.insert(id, vehicle.clone());

        Some(vehicle)
    }

    pub fn destroy_vehicle(
        &mut self,
        mut pending_state: MutexGuard<PendingVehicleDeletion>,
        vehicle: &Vehicle,
    ) {
        let ptr = vehicle.ptr.as_ref().unwrap();
        let id = get_vehicle_id(ptr.0);

        if self.vehicles.remove(&id).is_some() {
            unsafe {
                sdk::destroy_baseobject(
                    sdk::convert_vehicle_to_baseobject(ptr.0) as *mut sdk::IBaseObject
                )
            }
            crate::log!("[destroy_vehicle] ~gl~successfully removed vehicle from container");
        } else {
            crate::log_error!("destroy unknown vehicle: {id}");
        }
    }

    // TODO:
    // pub fn destroy_vehicle(&mut self, vehicle: Vehicle) {
    //     self.vehicles.remove(id);
    // }

    pub fn on_vehicle_create(&mut self, raw_ptr: RawVehiclePointer) {
        let id = get_vehicle_id(raw_ptr);

        // useless because of PendingVehicleCreation
        // // vehicle is created in this resource
        // if self.vehicles.get(&get_vehicle_id(vehicle)).is_some() {
        //     // TODO: TEST
        //     dbg!("vehicle is created here: {id}");
        //     return;
        // }

        crate::log_warn!("[on_vehicle_create] external vehicle: {id}");

        let vehicle: VehicleContainer = create_vehicle_container(id, raw_ptr);

        self.vehicles.insert(id, vehicle);
    }

    pub fn on_vehicle_destroy(&mut self, raw_vehicle: RawVehiclePointer) {
        let id = get_vehicle_id(raw_vehicle);
        crate::log_warn!("[on_vehicle_destroy] external vehicle: {id}");

        if self.vehicles.remove(&id).is_some() {
            crate::log!("[on_vehicle_destroy] ~gl~successfully removed vehicle from container");
        } else {
            crate::log_error!("[on_vehicle_destroy] unknown vehicle: {id}");
        }
    }
}

fn get_vehicle_id(raw_ptr: RawVehiclePointer) -> EntityId {
    unsafe { sdk::get_entity_id(sdk::convert_vehicle_to_entity(raw_ptr) as *mut sdk::IEntity) }
}

fn create_vehicle_container(id: u16, raw_ptr: RawVehiclePointer) -> VehicleContainer {
    Arc::new(Mutex::new(Vehicle {
        id,
        ptr: Some(VehiclePointer(raw_ptr)),
    }))
}
