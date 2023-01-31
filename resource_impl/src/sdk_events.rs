use std::collections::HashMap;

pub enum SDKEvent {
    ServerStarted(Box<dyn FnMut() + 'static + Send + Sync>),
    PlayerConnect(fn(player: usize)),
    PlayerDisconnect(fn(player: usize, reason: String)),
}

impl std::fmt::Debug for SDKEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SDKEvent (any)")
    }
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

pub type EventHandlersHashMap = HashMap<altv_sdk::EventType, SDKEvent>;

#[derive(Debug)]
pub struct SDKEventManager {
    handlers: EventHandlersHashMap,
}

impl SDKEventManager {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn toggle_event_type(&self, event_type: altv_sdk::EventType, state: bool) {
        unsafe {
            altv_sdk::ffi::toggle_event_type(event_type as u16, state);
        }
    }

    // intended for altv_module
    pub fn __get_handlers(&self) -> &EventHandlersHashMap {
        &self.handlers
    }
}
