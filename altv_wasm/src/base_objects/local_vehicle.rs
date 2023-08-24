use std::{cell::RefCell, collections::HashSet};

use altv_wasm_shared::BaseObjectPtr;
use crate::__imports;

#[derive(Debug, Default)]
pub struct VehicleManager {
    objects: Vec<LocalVehicle>,
}

impl VehicleManager {
    pub fn new(
        &mut self,
        model: u32,
        dimension: i32,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        rot_x: f32,
        rot_y: f32,
        rot_z: f32,
        use_streaming: bool,
        streaming_distance: u32,
    ) -> LocalVehicle {
        let ptr = __imports::create_local_vehicle(
            model,
            dimension,
            pos_x,
            pos_y,
            pos_z,
            rot_x,
            rot_y,
            rot_z,
            use_streaming,
            streaming_distance,
        );

        LOCAL_VEHICLE_STORE.with(|v| {
            v.borrow_mut().insert(ptr);
        });

        LocalVehicle { ptr }
    }

    pub fn all(&mut self) -> &[LocalVehicle] {
        LOCAL_VEHICLE_STORE.with(|v| {
            self.objects = v
                .borrow()
                .iter()
                .map(|ptr| LocalVehicle { ptr: *ptr })
                .collect();
        });

        &self.objects
    }

    pub fn destroy(&mut self, object: LocalVehicle) {
        LOCAL_VEHICLE_STORE.with(|v| {
            v.borrow_mut().remove(&object.ptr);
        });

        __imports::destroy_base_object(object.ptr);
    }
}

thread_local! {
    pub static LOCAL_VEHICLE_STORE: RefCell<HashSet<BaseObjectPtr>> = RefCell::new(HashSet::new());
}

pub fn vehicles() -> VehicleManager {
    VehicleManager::default()
}

#[derive(Debug)]
pub struct LocalVehicle {
    pub(crate) ptr: BaseObjectPtr,
}

impl LocalVehicle {
    pub fn fuel_level(&self) -> f32 {
        __imports::vehicle_get_fuel_level(self.ptr)
    }

    pub fn set_fuel_level(&self, value: f32) {
        __imports::vehicle_set_fuel_level(self.ptr, value);
    }

    pub fn destroy(self, manager: &mut VehicleManager) {
        manager.destroy(self);
    }
}
