use std::{collections::HashMap, fmt::Debug, cell::RefMut};

use altv_wasm_shared::EventType;

use crate::state::State;

pub mod contexts {
    use crate::base_objects::{any_vehicle::AnyVehicle, any_entity::AnyEntity};

    use super::*;

    pub(crate) enum EventContext {
        EnteredVehicle(EnteredVehicle),
        GameEntityCreate(GameEntityCreate),
        GameEntityDestroy(GameEntityDestroy),
    }

    impl EventContext {
        pub(crate) fn event_type(&self) -> EventType {
            match self {
                EventContext::EnteredVehicle(_) => EventType::PlayerEnterVehicle,
                EventContext::GameEntityCreate(_) => EventType::GameEntityCreate,
                EventContext::GameEntityDestroy(_) => EventType::GameEntityDestroy,
                _ => todo!(),
            }
        }
    }

    #[derive(Debug)]
    pub struct EnteredVehicle {
        pub vehicle: AnyVehicle,
        pub seat: u8,
    }

    #[derive(Debug)]
    pub struct GameEntityCreate {
        pub entity: AnyEntity,
    }

    #[derive(Debug)]
    pub struct GameEntityDestroy {
        pub entity: AnyEntity,
    }
}

pub enum EventHandler {
    EnteredVehicle(Box<dyn FnMut(&contexts::EnteredVehicle)>),
    GameEntityCreate(Box<dyn FnMut(&contexts::GameEntityCreate)>),
    GameEntityDestroy(Box<dyn FnMut(&contexts::GameEntityDestroy)>),
}

impl EventHandler {
    fn event_type(&self) -> EventType {
        match self {
            EventHandler::EnteredVehicle(_) => EventType::PlayerEnterVehicle,
            EventHandler::GameEntityCreate(_) => EventType::GameEntityCreate,
            EventHandler::GameEntityDestroy(_) => EventType::GameEntityDestroy,
            _ => todo!(),
        }
    }
}

impl Debug for EventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventHandler")
    }
}

type HandlerId = u64;

#[derive(Debug, Default)]
pub(crate) struct EventManager {
    handlers: HashMap<EventType, HashMap<HandlerId, EventHandler>>,
    handler_id: u64,
}

impl EventManager {
    pub(crate) fn add_handler(&mut self, handler: EventHandler) -> ScheduledHandlerRemoval {
        let ty = handler.event_type();
        let handlers = self.handlers.entry(ty).or_default();

        self.handler_id += 1;
        handlers.insert(self.handler_id, handler);

        (ty, self.handler_id)
    }

    pub(crate) fn call_handlers(
        &mut self,
        context: contexts::EventContext,
        schedule: RefMut<EventSchedule>,
    ) {
        self.remove_scheduled_handlers(schedule);

        let ty = context.event_type();
        let handlers = self.handlers.get_mut(&ty);
        let Some(handlers) = handlers else {
            logger::debug!("unhandled event: {ty:?}");
            return;
        };
        if handlers.values().count() == 0 {
            logger::debug!("unhandled event: {ty:?}");
            return;
        }

        for h in handlers.values_mut() {
            match h {
                EventHandler::EnteredVehicle(cb) => cb({
                    match &context {
                        contexts::EventContext::EnteredVehicle(c) => c,
                        _ => unreachable!(),
                    }
                }),
                EventHandler::GameEntityCreate(cb) => cb({
                    match &context {
                        contexts::EventContext::GameEntityCreate(c) => c,
                        _ => unreachable!(),
                    }
                }),
                EventHandler::GameEntityDestroy(cb) => cb({
                    match &context {
                        contexts::EventContext::GameEntityDestroy(c) => c,
                        _ => unreachable!(),
                    }
                }),
            }
        }
    }

    fn remove_scheduled_handlers(&mut self, mut schedule: RefMut<EventSchedule>) {
        for (ty, id) in std::mem::take(&mut schedule.removes) {
            let Some(handlers) = self.handlers.get_mut(&ty) else {
                logger::error!("remove_scheduled_handlers ty: {ty:?} does not have handlers");
                continue;
            };
            if let None = handlers.remove(&id) {
                logger::error!(
                    "remove_scheduled_handlers failed to remove handler ty: {ty:?} id: {id:?}"
                );
            }
        }
    }
}

type ScheduledHandlerRemoval = (EventType, HandlerId);

#[derive(Debug, Default)]
pub(crate) struct EventSchedule {
    removes: Vec<ScheduledHandlerRemoval>,
}

impl EventSchedule {
    pub fn add_handler_to_remove(&mut self, remove: ScheduledHandlerRemoval) {
        self.removes.push(remove);
    }
}

#[derive(Debug)]
pub struct EventController {
    removal: ScheduledHandlerRemoval,
}

impl EventController {
    pub fn destroy(self) {
        State::with_event_schedule_mut(|mut schedule, _| {
            schedule.add_handler_to_remove(self.removal);
        });
    }
}

pub fn add_handler(handler: EventHandler) -> EventController {
    State::with_events_mut(|mut events, _| {
        let removal = events.add_handler(handler);
        EventController { removal }
    })
}

pub fn on_game_entity_create(
    handler: impl FnMut(&contexts::GameEntityCreate) + 'static,
) -> EventController {
    add_handler(EventHandler::GameEntityCreate(Box::new(handler)))
}
