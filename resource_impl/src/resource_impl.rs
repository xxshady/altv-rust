use crate::{
    events::{Event, EventManager, PublicEventType},
    timers::{TimerCallback, TimerManager},
    vehicle::VehicleManager,
};
use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard};

// should not be used in altv_module
static RESOURCE_IMPL_INSTANCE: OnceCell<Mutex<ResourceImpl>> = OnceCell::new();

#[derive(Debug)]
pub struct ResourceImpl {
    pub full_main_path: String,
    timers: TimerManager,
    events: EventManager,
    vehicles: VehicleManager,
}

impl ResourceImpl {
    pub fn init(full_main_path: String) {
        let instance = ResourceImpl {
            full_main_path,
            timers: TimerManager::new(),
            events: EventManager::new(),
            vehicles: VehicleManager::new(),
        };

        RESOURCE_IMPL_INSTANCE
            .set(Mutex::new(instance))
            .expect("RESOURCE_IMPL_INSTANCE.set() failed");
    }

    pub fn get_mutex() -> &'static Mutex<Self> {
        RESOURCE_IMPL_INSTANCE.get().unwrap()
    }

    pub fn instance() -> MutexGuard<'static, Self> {
        RESOURCE_IMPL_INSTANCE
            .get()
            .expect("RESOURCE_IMPL_INSTANCE.get() failed")
            .try_lock()
            .expect("RESOURCE_IMPL_INSTANCE try_lock failed")
    }

    pub fn create_timer(&mut self, callback: Box<TimerCallback>, millis: u64, once: bool) {
        self.timers.create(callback, millis, once);
    }

    // intended for altv_module
    pub fn __on_tick(&mut self) {
        self.timers.__process_timers();
    }

    pub fn add_event_handler(
        &mut self,
        public_type: PublicEventType,
        sdk_type: altv_sdk::EventType,
        event: Event,
    ) {
        self.events.add_handler(public_type, sdk_type, event);
    }

    // intended for altv_module
    pub fn __on_sdk_event(
        &mut self,
        event_type: altv_sdk::EventType,
        event: *const altv_sdk::ffi::CEvent,
    ) {
        self.events.on_sdk_event(event_type, event);
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
    ) -> Option<crate::vehicle::VehicleContainer> {
        self.vehicles.create_vehicle(model, x, y, z, rx, ry, rz)
    }

    // intended for altv_module
    pub fn __on_vehicle_create(&mut self, vehicle: *mut altv_sdk::ffi::IVehicle) {
        self.vehicles.on_vehicle_create(vehicle);
    }

    // TODO:
    // intended for altv_module
    // pub fn __on_custom_event(&mut self, event_type: PublicEventType) {
    //     self.events.on_custom_event(event_type);
    // }
}
