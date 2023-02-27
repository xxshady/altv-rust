use crate::{
    base_object, base_object_maps,
    entity::{self, Entity},
    events::{self, Event, PublicEventType},
    local_script_events, player, timers, vehicle,
};
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub type ResourceImplRef = Rc<RefCell<ResourceImpl>>;

thread_local! {
    pub static RESOURCE_IMPL_INSTANCE: RefCell<Option<ResourceImplRef>> = RefCell::new(None);
}

pub fn with_resource_impl<F, R>(f: F) -> R
where
    F: FnOnce(Ref<ResourceImpl>) -> R,
{
    RESOURCE_IMPL_INSTANCE.with(|v| f(v.borrow().as_ref().unwrap().borrow()))
}

// intended for altv_module
#[derive(Debug)]
pub struct ResourceImpl {
    timers: RefCell<timers::TimerManager>,
    timer_schedule_state: RefCell<timers::ScheduleState>,
    base_object_creation: RefCell<base_object::PendingBaseObjectCreation>,
    base_object_deletion: RefCell<base_object::PendingBaseObjectDeletion>,
    base_objects: RefCell<base_object::BaseObjectManager>,
    entities: RefCell<entity::EntityManager>,
    events: RefCell<events::EventManager>,
    player_base_object_map: RefCell<base_object_maps::PlayerBaseObjectMap>,
    local_script_events: RefCell<local_script_events::LocalEventManager>,
}

impl ResourceImpl {
    pub fn new() -> Self {
        Self {
            timer_schedule_state: RefCell::new(timers::ScheduleState::new()),
            timers: RefCell::new(timers::TimerManager::new()),
            base_object_creation: RefCell::new(base_object::PendingBaseObjectCreation::new()),
            base_object_deletion: RefCell::new(base_object::PendingBaseObjectDeletion::new()),
            base_objects: RefCell::new(base_object::BaseObjectManager::new()),
            entities: RefCell::new(entity::EntityManager::new()),
            events: RefCell::new(events::EventManager::new()),
            player_base_object_map: RefCell::new(base_object_maps::PlayerBaseObjectMap::new()),
            local_script_events: RefCell::new(local_script_events::LocalEventManager::new()),
        }
    }

    pub fn init(resource_impl: ResourceImplRef) {
        RESOURCE_IMPL_INSTANCE.with(|instance| instance.borrow_mut().replace(resource_impl));
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
                self.player_base_object_map
                    .borrow_mut()
                    .add_base_object(Rc::clone(&player));
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

        let remove_entity_from_pool = |base_object_borrow: &Ref<dyn base_object::BaseObject>| {
            self.entities
                .borrow_mut()
                .on_destroy(base_object_borrow.ptr().to_entity().unwrap());
        };

        let mut base_objects = self.base_objects.borrow_mut();
        let base_object = base_objects.get_by_raw_ptr(raw_ptr);
        if let Some(base_object) = base_object {
            let base_object_borrow = base_object.borrow();

            use altv_sdk::BaseObjectType::*;
            match base_object_borrow.base_type() {
                PLAYER => {
                    remove_entity_from_pool(&base_object_borrow);
                    self.player_base_object_map
                        .borrow_mut()
                        .remove_base_object(base_object_borrow.ptr().get().unwrap());
                }
                VEHICLE => {
                    remove_entity_from_pool(&base_object_borrow);
                }
                _ => todo!(),
            };
            drop(base_object_borrow);
            base_objects.on_destroy(Rc::clone(&base_object));
        } else {
            crate::log_error!("__on_base_object_destroy unknown base object: {raw_ptr:?}");
        }
    }

    pub fn __on_sdk_event(
        &self,
        event_type: altv_sdk::EventType,
        event: *const altv_sdk::ffi::alt::CEvent,
    ) {
        self.events.borrow_mut().__on_sdk_event(
            self.player_base_object_map.borrow(),
            self.local_script_events.borrow_mut(),
            event_type,
            event,
        );
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

    pub(crate) fn borrow_entities(&self) -> Ref<entity::EntityManager> {
        self.entities.borrow()
    }

    pub(crate) fn borrow_mut_entities(&self) -> RefMut<entity::EntityManager> {
        self.entities.borrow_mut()
    }
}

pub fn timers_create(callback: Box<timers::TimerCallback>, millis: u64, once: bool) {
    with_resource_impl(|instance| {
        timers::create(
            instance.timer_schedule_state.borrow_mut(),
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
    with_resource_impl(|instance| {
        instance
            .events
            .borrow_mut()
            .add_handler(public_type, sdk_type, event);
    });
}

pub fn on(
    event_name: String,
    handler: impl FnMut(local_script_events::LocalEventArgs) -> anyhow::Result<()> + 'static,
) {
    with_resource_impl(|instance| {
        instance
            .local_script_events
            .borrow_mut()
            .add_handler(event_name, Box::new(handler))
    });
}
