use std::collections::HashMap;

#[derive(Debug)]
pub enum SDKEvent {
    ServerStarted(fn()),
    PlayerConnect(fn(player: usize)),
    PlayerDisconnect(fn(player: usize, reason: String)),
}

impl SDKEvent {
    fn to_sdk_type(&self) -> altv_sdk::EventType {
        use altv_sdk::EventType::*;
        match self {
            SDKEvent::ServerStarted(_) => SERVER_STARTED,
            SDKEvent::PlayerConnect(_) => PLAYER_CONNECT,
            SDKEvent::PlayerDisconnect(_) => PLAYER_DISCONNECT,
        }
    }
}

#[derive(Debug)]
pub struct SDKEventManager {
    handlers: HashMap<altv_sdk::EventType, Vec<SDKEvent>>,
}

pub type EventHandlersHashMap = HashMap<altv_sdk::EventType, Vec<SDKEvent>>;

impl SDKEventManager {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler(&mut self, event: SDKEvent) {
        let event_type = event.to_sdk_type();
        let handlers = self.handlers.get_mut(&event_type);

        if let Some(handlers) = handlers {
            handlers.push(event);
        } else {
            unsafe {
                altv_sdk::ffi::toggle_event_type(event_type as u16, true);
            }
            self.handlers.insert(event_type, vec![event]);
        }
    }

    // intended for altv_module
    pub fn __get_handlers(&self) -> &EventHandlersHashMap {
        &self.handlers
    }
}
