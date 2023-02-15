use crate::{
    base_object,
    entity::{self, Entity},
    events::{self, Event, PublicEventType},
    player, timers, vehicle,
};
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
    thread::LocalKey,
};

pub type ResourceImplContainer = &'static LocalKey<RefCell<ResourceImpl>>;

thread_local! {
    pub static RESOURCE_IMPL_INSTANCE: RefCell<ResourceImpl> = RefCell::new(ResourceImpl::new());
}

// intended for altv_module
#[derive(Debug)]
pub struct ResourceImpl {
    pub full_main_path: String,
    timers: RefCell<timers::TimerManager>,
    timer_schedule_state: RefCell<timers::ScheduleState>,
    base_object_creation: RefCell<base_object::PendingBaseObjectCreation>,
    base_object_deletion: RefCell<base_object::PendingBaseObjectDeletion>,
    base_objects: RefCell<base_object::BaseObjectManager>,
    entities: RefCell<entity::EntityManager>,
    players: RefCell<player::PlayerManager>,
    events: RefCell<events::EventManager>,
}

impl ResourceImpl {
    pub(crate) fn new() -> Self {
        Self {
            full_main_path: "".into(),
            timer_schedule_state: RefCell::new(timers::ScheduleState::new()),
            timers: RefCell::new(timers::TimerManager::new()),
            base_object_creation: RefCell::new(base_object::PendingBaseObjectCreation::new()),
            base_object_deletion: RefCell::new(base_object::PendingBaseObjectDeletion::new()),
            base_objects: RefCell::new(base_object::BaseObjectManager::new()),
            entities: RefCell::new(entity::EntityManager::new()),
            players: RefCell::new(player::PlayerManager::new()),
            events: RefCell::new(events::EventManager::new()),
        }
    }

    pub fn init(full_main_path: String) -> ResourceImplContainer {
        RESOURCE_IMPL_INSTANCE.with(|instance| {
            instance.borrow_mut().full_main_path = full_main_path;
        });

        &RESOURCE_IMPL_INSTANCE
    }

    pub fn __on_tick(&self) {
        self.timers
            .borrow_mut()
            .process_timers(self.timer_schedule_state.borrow_mut());
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn __on_base_object_create(
        &self,
        raw_ptr: base_object::RawBaseObjectPointer,
        base_object_type: altv_sdk::BaseObjectType,
    ) {
        if self.base_object_creation.try_borrow_mut().is_err() {
            crate::log_warn!("__on_base_object_create pending creation, skip");
            return;
        }

        let add_entity_to_pool = |entity: entity::EntityWrapper| {
            self.entities.borrow_mut().on_create(
                match &entity {
                    entity::EntityWrapper::Player(p) => p.borrow().id().unwrap(),
                    entity::EntityWrapper::Vehicle(p) => p.borrow().id().unwrap(),
                },
                entity,
            )
        };

        use altv_sdk::BaseObjectType::*;
        let base_object: base_object::BaseObjectContainer = match base_object_type {
            VEHICLE => {
                let vehicle = vehicle::create_vehicle_container(raw_ptr);
                add_entity_to_pool(entity::EntityWrapper::Vehicle(Rc::clone(&vehicle)));
                vehicle
            }
            PLAYER => {
                let player = player::create_player_container(raw_ptr);
                add_entity_to_pool(entity::EntityWrapper::Player(Rc::clone(&player)));
                self.players.borrow_mut().add_player(Rc::clone(&player));
                player
            }
            _ => todo!(),
        };

        self.base_objects
            .borrow_mut()
            .on_create(raw_ptr, base_object);
    }

    pub fn __on_base_object_destroy(&self, raw_ptr: base_object::RawBaseObjectPointer) {
        if self.base_object_deletion.try_borrow_mut().is_err() {
            crate::log_warn!("__on_base_object_destroy pending deletion, skip");
            return;
        }

        let mut base_objects = self.base_objects.borrow_mut();
        let base_object = base_objects.get_by_raw_ptr(raw_ptr);
        if let Some(base_object) = base_object {
            use altv_sdk::BaseObjectType::*;
            let base_object_borrow = base_object.borrow();
            match base_object_borrow.base_type() {
                VEHICLE | PLAYER => {
                    self.entities
                        .borrow_mut()
                        .on_destroy(base_object_borrow.ptr().to_entity().unwrap());
                    // TEST
                    // TODO: check if baseobject is player and only then remove it from player pool
                    self.players
                        .borrow_mut()
                        .remove_player(base_object_borrow.ptr().get().unwrap());
                }
                _ => todo!(),
            };
            drop(base_object_borrow);
            base_objects.on_destroy(Rc::clone(&base_object));
        } else {
            crate::log_error!("__on_base_object_destroy unknown base object: {raw_ptr:?}");
        }
    }

    pub(crate) fn borrow_mut_base_object_deletion(
        &self,
    ) -> RefMut<base_object::PendingBaseObjectDeletion> {
        self.base_object_deletion.borrow_mut()
    }

    pub(crate) fn borrow_mut_base_object_creation(
        &self,
    ) -> RefMut<base_object::PendingBaseObjectCreation> {
        self.base_object_creation.borrow_mut()
    }

    pub(crate) fn borrow_mut_base_objects(&self) -> RefMut<base_object::BaseObjectManager> {
        self.base_objects.borrow_mut()
    }

    pub fn __on_sdk_event(
        &self,
        event_type: altv_sdk::EventType,
        event: *const altv_sdk::ffi::CEvent,
    ) {
        self.events.borrow_mut().on_sdk_event(
            self.base_objects.borrow(),
            self.players.borrow(),
            event_type,
            event,
        );
    }
}

pub fn timers_create(callback: Box<timers::TimerCallback>, millis: u64, once: bool) {
    RESOURCE_IMPL_INSTANCE.with(|instance| {
        timers::create(
            instance.borrow().timer_schedule_state.borrow_mut(),
            callback,
            millis,
            once,
        )
    });
}

pub fn add_event_handler(
    public_type: PublicEventType,
    sdk_type: altv_sdk::EventType,
    event: Event,
) {
    RESOURCE_IMPL_INSTANCE.with(|instance| {
        instance
            .borrow()
            .events
            .borrow_mut()
            .add_handler(public_type, sdk_type, event);
    });
}
