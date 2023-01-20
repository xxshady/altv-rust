use once_cell::sync::OnceCell;
use std::{collections::HashMap, sync::Mutex};

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

pub struct SDKEventManager {
    handlers: HashMap<altv_sdk::EventType, Vec<SDKEvent>>,
}

// TEST
unsafe impl Sync for SDKEventManager {}
unsafe impl Send for SDKEventManager {}

pub static SDK_EVENT_MANAGER: OnceCell<Mutex<SDKEventManager>> = OnceCell::new();

impl SDKEventManager {
    fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn instance() -> &'static Mutex<SDKEventManager> {
        SDK_EVENT_MANAGER.get_or_init(|| Mutex::new(SDKEventManager::new()))
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

    pub fn get_handlers(&self) -> &HashMap<altv_sdk::EventType, Vec<SDKEvent>> {
        &self.handlers
    }
}
