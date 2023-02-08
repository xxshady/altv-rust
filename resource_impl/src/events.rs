use std::collections::{HashMap, HashSet};

use crate::{player::Player, sdk_events::SDKEventManager};

pub use altv_sdk::EventType as SDKEventType;

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum PublicEventType {
    ServerStarted,
    PlayerConnect,
    PlayerDisconnect,
    ResourceStop,
    BaseObjectCreate,
}

impl From<SDKEventType> for PublicEventType {
    fn from(sdk_type: SDKEventType) -> Self {
        use SDKEventType::*;
        match sdk_type {
            SERVER_STARTED => Self::ServerStarted,
            PLAYER_CONNECT => Self::PlayerConnect,
            PLAYER_DISCONNECT => Self::PlayerDisconnect,
            _ => {
                panic!("Cannot convert sdk event type: {sdk_type:?} to PublicEventType");
            }
        }
    }
}

#[derive(Debug)]
pub struct ServerStartedController {}

#[derive(Debug)]
pub struct PlayerConnectController {
    pub player: Player,
}

#[derive(Debug)]
pub struct PlayerDisconnectController {
    pub player: Player,
    pub reason: String,
}

#[derive(Debug)]
pub struct BaseObjectCreateController {}

#[repr(u16)]
pub enum Event {
    ServerStarted(Box<dyn FnMut(ServerStartedController) + Send + Sync>),
    PlayerConnect(Box<dyn FnMut(PlayerConnectController) + Send + Sync>),
    PlayerDisconnect(Box<dyn FnMut(PlayerDisconnectController) + Send + Sync>),
    BaseObjectCreate(Box<dyn FnMut(BaseObjectCreateController) + Send + Sync>),
    ResourceStop(fn()),
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO:
        write!(f, "Event <any>")
    }
}

// TODO:
// pub enum CustomEventPayload {
//     BaseObjectCreate(base_object_type: altv_sdk::BaseObjectType, pointer: *const altv_sdk::ffi::IBaseObject),
// }

#[derive(Debug)]
pub struct EventManager {
    public_handlers: HashMap<PublicEventType, Vec<Event>>,
    enabled_sdk_events: HashSet<SDKEventType>,
    sdk_events_manager: SDKEventManager,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            enabled_sdk_events: HashSet::new(),
            public_handlers: HashMap::new(),
            sdk_events_manager: SDKEventManager::new(),
        }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn on_sdk_event(&mut self, event_type: SDKEventType, event: *const altv_sdk::ffi::CEvent) {
        let handlers = self.public_handlers.get_mut(&event_type.into());

        if let Some(handlers) = handlers {
            for h in handlers {
                use Event::*;
                match h {
                    ServerStarted(callback) => callback(ServerStartedController {}),
                    PlayerConnect(callback) => {
                        callback(PlayerConnectController { player: todo!() })
                    }
                    PlayerDisconnect(callback) => callback(PlayerDisconnectController {
                        player: todo!(),
                        reason: unsafe { altv_sdk::ffi::get_event_reason(event) }.to_string(),
                    }),
                    _ => todo!(),
                }
            }
        } else {
            crate::log_error!(
                "[events.on_sdk_event] received event without handlers: {:?}",
                event_type
            );
        }
    }

    // TODO:
    // pub fn on_custom_event(&mut self, event_type: PublicEventType, payload: CustomEventPayload) {
    //     let handlers = self.public_handlers.get_mut(&event_type);

    //     if let Some(handlers) = handlers {
    //         for h in handlers {
    //             use Event::*;
    //             match h {
    //                 BaseObjectCreate(h) => h(BaseObjectCreateController {}),
    //                 _ => todo!(),
    //             }
    //         }
    //     } else {
    //         crate::log_error!(
    //             "[events.on_custom_event] received event without handlers: {:?}",
    //             event_type
    //         );
    //     }

    //     // internal stuff

    // }

    pub fn add_handler(
        &mut self,
        public_type: PublicEventType,
        sdk_type: SDKEventType,
        event: Event,
    ) {
        crate::log!("events add_handler event public_type: {public_type:?}");

        if sdk_type != SDKEventType::NONE && self.enabled_sdk_events.get(&sdk_type).is_none() {
            crate::log!("sdk_type: {sdk_type:?} | enabling it in sdk");

            self.enabled_sdk_events.insert(sdk_type);
            self.sdk_events_manager.toggle_event_type(sdk_type, true);
        }

        let handlers = self.public_handlers.get_mut(&public_type);

        if let Some(handlers) = handlers {
            handlers.push(event);
        } else {
            self.public_handlers.insert(public_type, vec![event]);
        }
    }
}
