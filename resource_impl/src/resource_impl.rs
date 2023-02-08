use crate::{
    base_object::{self, BaseObjectContainer, RawBaseObjectPointer},
    entity::{self, Entity},
    events::{self, Event, PublicEventType},
    timers, vehicle,
};
use once_cell::sync::OnceCell;
use std::sync::Mutex;

// these statics should not be used directly in altv_module,
// but via ResourceImpl instance
static TIMER_MANAGER_INSTANCE: OnceCell<Mutex<timers::TimerManager>> = OnceCell::new();
static TIMER_SCHEDULE_STATE_INSTANCE: OnceCell<Mutex<timers::ScheduleState>> = OnceCell::new();
static EVENT_MANAGER_INSTANCE: OnceCell<Mutex<events::EventManager>> = OnceCell::new();

macro_rules! init_static {
    ($static_var: path, $manager: ty) => {{
        $static_var.set(Mutex::new(<$manager>::new())).unwrap();
        $static_var.get().unwrap()
    }};
}

// intended for altv_module
#[derive(Debug)]
pub struct ResourceImpl {
    pub full_main_path: String,
    timers: &'static Mutex<timers::TimerManager>,
    timer_schedule_state: &'static Mutex<timers::ScheduleState>,
    events: &'static Mutex<events::EventManager>,
    vehicles: &'static Mutex<vehicle::VehicleManager>,
    base_object_creation: &'static Mutex<base_object::PendingBaseObjectCreation>,
    base_object_deletion: &'static Mutex<base_object::PendingBaseObjectDeletion>,
    base_objects: &'static Mutex<base_object::BaseObjectManager>,
    entities: &'static Mutex<entity::EntityManager>,
}

impl ResourceImpl {
    pub fn init(full_main_path: String) -> ResourceImpl {
        let instance = ResourceImpl {
            full_main_path,
            timers: init_static!(TIMER_MANAGER_INSTANCE, timers::TimerManager),
            timer_schedule_state: init_static!(
                TIMER_SCHEDULE_STATE_INSTANCE,
                timers::ScheduleState
            ),
            events: init_static!(EVENT_MANAGER_INSTANCE, events::EventManager),
            vehicles: init_static!(vehicle::VEHICLE_MANAGER_INSTANCE, vehicle::VehicleManager),
            base_object_creation: init_static!(
                base_object::BASE_OBJECT_CREATION_INSTANCE,
                base_object::PendingBaseObjectCreation
            ),
            base_object_deletion: init_static!(
                base_object::BASE_OBJECT_DELETION_INSTANCE,
                base_object::PendingBaseObjectDeletion
            ),
            base_objects: init_static!(
                base_object::BASE_OBJECT_MANAGER_INSTANCE,
                base_object::BaseObjectManager
            ),
            entities: init_static!(entity::ENTITY_MANAGER_INSTANCE, entity::EntityManager),
        };

        instance
    }

    pub fn __on_sdk_event(
        &self,
        event_type: altv_sdk::EventType,
        event: *const altv_sdk::ffi::CEvent,
    ) {
        self.events
            .try_lock()
            .unwrap()
            .on_sdk_event(event_type, event);
    }

    pub fn __on_tick(&self) {
        self.timers
            .try_lock()
            .unwrap()
            .process_timers(self.timer_schedule_state.try_lock().unwrap());
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn __on_base_object_create(
        &self,
        raw_ptr: RawBaseObjectPointer,
        base_object_type: altv_sdk::BaseObjectType,
    ) {
        if self.base_object_creation.try_lock().is_err() {
            crate::log_warn!("__on_base_object_create pending creation, skip");
            return;
        }

        use altv_sdk::BaseObjectType::*;
        let base_object: BaseObjectContainer = match base_object_type {
            VEHICLE => {
                let vehicle = vehicle::VehicleManager::create_vehicle_container(unsafe {
                    altv_sdk::ffi::convert_baseobject_to_vehicle(raw_ptr)
                });
                self.entities.try_lock().unwrap().on_create(
                    vehicle.try_lock().unwrap().id().unwrap(),
                    entity::EntityWrapper::Vehicle(vehicle.clone()),
                );
                vehicle
            }
            _ => todo!(),
        };

        self.base_objects
            .try_lock()
            .unwrap()
            .on_create(raw_ptr, base_object);
    }

    pub fn __on_base_object_destroy(&self, raw_ptr: RawBaseObjectPointer) {
        if self.base_object_deletion.try_lock().is_err() {
            crate::log_warn!("__on_base_object_destroy pending deletion, skip");
            return;
        }

        let mut base_objects = self.base_objects.try_lock().unwrap();
        let base_object = base_objects.get_by_raw_ptr(raw_ptr);
        if let Some(base_object) = base_object {
            use altv_sdk::BaseObjectType::*;
            let base_object_guard = base_object.try_lock().unwrap();
            match base_object_guard.base_type() {
                VEHICLE => {
                    self.entities
                        .try_lock()
                        .unwrap()
                        .on_destroy(base_object_guard.ptr().to_entity().unwrap());
                }
                _ => todo!(),
            };
            drop(base_object_guard);
            base_objects.on_destroy(base_object.clone());
        } else {
            crate::log_error!("__on_base_object_destroy unknown base object: {raw_ptr:?}");
        }
    }

    // pub fn __on_vehicle_create(&self, vehicle: *mut altv_sdk::ffi::IVehicle) {
    //     if self.vehicle_creation.try_lock().is_err() {
    //         crate::log_warn!("__on_vehicle_create pending creation, skip");
    //         return;
    //     }
    //     self.vehicles.try_lock().unwrap().on_vehicle_create(vehicle);
    // }

    // pub fn __on_vehicle_destroy(&self, vehicle: *mut altv_sdk::ffi::IVehicle) {
    //     if self.vehicle_deletion.try_lock().is_err() {
    //         crate::log_warn!("__on_vehicle_destroy pending deletion, skip");
    //         return;
    //     }
    //     self.vehicles
    //         .try_lock()
    //         .unwrap()
    //         .on_vehicle_destroy(vehicle);
    // }
}

pub fn timers_create(callback: Box<timers::TimerCallback>, millis: u64, once: bool) {
    let state = TIMER_SCHEDULE_STATE_INSTANCE
        .get()
        .unwrap()
        .try_lock()
        .unwrap();

    timers::create(state, callback, millis, once);
}

pub fn create_timer(callback: Box<timers::TimerCallback>, millis: u64, once: bool) {
    let state = TIMER_SCHEDULE_STATE_INSTANCE
        .get()
        .unwrap()
        .try_lock()
        .unwrap();

    timers::create(state, callback, millis, once);
}

pub fn add_event_handler(
    public_type: PublicEventType,
    sdk_type: altv_sdk::EventType,
    event: Event,
) {
    EVENT_MANAGER_INSTANCE
        .get()
        .unwrap()
        .try_lock()
        .unwrap()
        .add_handler(public_type, sdk_type, event);
}
