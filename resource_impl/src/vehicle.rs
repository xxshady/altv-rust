use altv_sdk::ffi as sdk;
use once_cell::sync::OnceCell;
use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard},
};

type EntityId = u16;
type RawVehiclePointer = *mut sdk::IVehicle;

pub(crate) static VEHICLE_MANAGER_INSTANCE: OnceCell<Mutex<VehicleManager>> = OnceCell::new();
pub(crate) static VEHICLE_CREATION_INSTANCE: OnceCell<Mutex<PendingVehicleCreation>> =
    OnceCell::new();
pub(crate) static VEHICLE_DELETION_INSTANCE: OnceCell<Mutex<PendingVehicleDeletion>> =
    OnceCell::new();

#[derive(Debug)]
struct VehiclePointer(Option<RawVehiclePointer>);

impl VehiclePointer {
    pub fn get(&self) -> Result<RawVehiclePointer, &str> {
        if let Some(raw) = self.0 {
            Ok(raw)
        } else {
            Err("Raw vehicle pointer is none")
        }
    }
}

// TEST
unsafe impl Send for VehiclePointer {}
unsafe impl Sync for VehiclePointer {}

#[derive(Debug)]
pub struct Vehicle {
    id: EntityId,
    ptr: VehiclePointer,
}

impl Vehicle {
    pub fn new(
        model: u32,
        x: f32,
        y: f32,
        z: f32,
        rx: f32,
        ry: f32,
        rz: f32,
    ) -> Option<VehicleContainer> {
        VEHICLE_MANAGER_INSTANCE
            .get()
            .unwrap()
            .try_lock()
            .unwrap()
            .create_vehicle(
                VEHICLE_CREATION_INSTANCE.get().unwrap().try_lock().unwrap(),
                model,
                x,
                y,
                z,
                rx,
                ry,
                rz,
            )
    }

    pub fn get_by_id(id: EntityId) -> Option<VehicleContainer> {
        let manager = VEHICLE_MANAGER_INSTANCE.get().unwrap().try_lock().unwrap();
        let maybe_vehicle = manager.vehicles.get(&id);
        maybe_vehicle.cloned()
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn valid(&self) -> bool {
        self.ptr.get().is_ok()
    }

    pub fn set_secondary_color(&self, color: u8) -> Result<(), &str> {
        unsafe { sdk::set_vehicle_primary_color(self.ptr.get()?, color) };
        Ok(())
    }

    pub fn get_secondary_color(&self) -> Result<u8, &str> {
        Ok(unsafe { sdk::get_vehicle_primary_color(self.ptr.get()?) })
    }

    pub fn destroy(&mut self) -> Result<(), String> {
        if self.ptr.get().is_err() {
            return Err("Vehicle is already destroyed".to_string());
        }

        VEHICLE_MANAGER_INSTANCE
            .get()
            .unwrap()
            .try_lock()
            .unwrap()
            .destroy_vehicle(
                VEHICLE_DELETION_INSTANCE.get().unwrap().try_lock().unwrap(),
                self,
            );
        self.ptr.0 = None;
        Ok(())
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

    pub fn destroy_vehicle(&mut self, _: MutexGuard<PendingVehicleDeletion>, vehicle: &Vehicle) {
        let raw_ptr = vehicle.ptr.get().unwrap();
        let id = get_vehicle_id(raw_ptr);

        if self.vehicles.remove(&id).is_some() {
            unsafe {
                sdk::destroy_baseobject(
                    sdk::convert_vehicle_to_baseobject(raw_ptr) as *mut sdk::IBaseObject
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
        crate::log_warn!("[on_vehicle_destroy] vehicle: {id}");

        if let Some(vehicle) = self.vehicles.get_mut(&id) {
            vehicle
                .try_lock()
                .expect("[on_vehicle_destroy] vehicle: {id} failed to lock")
                .ptr
                .0 = None;
            self.vehicles.remove(&id);
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
        ptr: VehiclePointer(Some(raw_ptr)),
    }))
}
