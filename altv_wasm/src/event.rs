use std::{collections::HashMap, fmt::Debug};

use altv_wasm_shared::EventType;

use crate::{Vehicle, state::State};

pub mod contexts {
    use super::*;

    pub(crate) enum EventContext {
        EnteredVehicle(EnteredVehicle),
    }

    impl EventContext {
        pub(crate) fn event_type(&self) -> EventType {
            match self {
                EventContext::EnteredVehicle(_) => EventType::PlayerEnterVehicle,
                _ => todo!(),
            }
        }
    }

    #[derive(Debug)]
    pub struct EnteredVehicle {
        pub vehicle: Vehicle,
        pub seat: u8,
    }
}

pub enum EventHandler {
    EnteredVehicle(Box<dyn FnMut(&contexts::EnteredVehicle)>),
}

impl EventHandler {
    fn event_type(&self) -> EventType {
        match self {
            EventHandler::EnteredVehicle(_) => EventType::PlayerEnterVehicle,
            _ => todo!(),
        }
    }
}

impl Debug for EventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventHandler")
    }
}

#[derive(Debug, Default)]
pub(crate) struct EventManager {
    handlers: HashMap<EventType, Vec<EventHandler>>,
}

impl EventManager {
    pub(crate) fn add_handler(&mut self, handler: EventHandler) {
        let ty = handler.event_type();
        let handlers = self.handlers.entry(ty).or_default();
        handlers.push(handler);
    }

    pub(crate) fn call_handlers(&mut self, context: contexts::EventContext) {
        let ty = context.event_type();
        let handlers = self.handlers.get_mut(&ty);
        let Some(handlers) = handlers else {
            return;
        };
        for h in handlers {
            match h {
                EventHandler::EnteredVehicle(cb) => cb({
                    match &context {
                        contexts::EventContext::EnteredVehicle(c) => c,
                        _ => unreachable!(),
                    }
                }),
            }
        }
    }
}

pub fn add_handler(handler: EventHandler) {
    State::with_events_mut(|mut events, _| {
        events.add_handler(handler);
    });
}
