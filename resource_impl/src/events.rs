use std::{
    cell::Ref,
    collections::{HashMap, HashSet},
};

use crate::{
    base_object_maps::PlayerBaseObjectMap, helpers::get_player_from_event, player::PlayerContainer,
};

pub use altv_sdk::EventType as SDKEventType;

use altv_sdk::ffi as sdk;

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum PublicEventType {
    ServerStarted,
    PlayerConnect,
    PlayerDisconnect,
    ResourceStop,
    BaseObjectCreate,
    ConsoleCommand,
}

impl From<SDKEventType> for PublicEventType {
    fn from(sdk_type: SDKEventType) -> Self {
        use SDKEventType::*;
        match sdk_type {
            SERVER_STARTED => Self::ServerStarted,
            PLAYER_CONNECT => Self::PlayerConnect,
            PLAYER_DISCONNECT => Self::PlayerDisconnect,
            RESOURCE_STOP => Self::ResourceStop,
            CONSOLE_COMMAND_EVENT => Self::ConsoleCommand,
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
    pub player: PlayerContainer,
}

#[derive(Debug)]
pub struct PlayerDisconnectController {
    pub player: PlayerContainer,
    pub reason: String,
}

#[derive(Debug)]
pub struct BaseObjectCreateController {}

#[derive(Debug)]
pub struct ConsoleCommandController {
    pub name: String,
    pub args: Vec<String>,
}

#[repr(u16)]
pub enum Event {
    ServerStarted(Box<dyn FnMut(ServerStartedController)>),
    PlayerConnect(Box<dyn FnMut(PlayerConnectController)>),
    PlayerDisconnect(Box<dyn FnMut(PlayerDisconnectController)>),
    BaseObjectCreate(Box<dyn FnMut(BaseObjectCreateController)>),
    ConsoleCommand(Box<dyn FnMut(ConsoleCommandController)>),
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
//     BaseObjectCreate(base_object_type: altv_sdk::BaseObjectType, pointer: *const sdk::alt::IBaseObject),
// }

#[derive(Debug)]
pub(crate) struct EventManager {
    public_handlers: HashMap<PublicEventType, Vec<Event>>,
    enabled_sdk_events: HashSet<SDKEventType>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            enabled_sdk_events: HashSet::new(),
            public_handlers: HashMap::new(),
        }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn __on_sdk_event(
        &mut self,
        players: Ref<PlayerBaseObjectMap>,
        event_type: SDKEventType,
        event: *const sdk::alt::CEvent,
    ) {
        // TEST
        crate::log_warn!("[events.on_sdk_event] received event: {:?}", event_type);

        let handlers = self.public_handlers.get_mut(&event_type.into());

        if let Some(handlers) = handlers {
            for h in handlers {
                use Event::*;
                match h {
                    ServerStarted(callback) => callback(ServerStartedController {}),
                    PlayerConnect(callback) => callback(PlayerConnectController {
                        player: get_player_from_event(event, &players),
                    }),
                    PlayerDisconnect(callback) => callback(PlayerDisconnectController {
                        player: get_player_from_event(event, &players),
                        reason: unsafe { sdk::get_event_reason(event) }.to_string(),
                    }),
                    ConsoleCommand(callback) => callback(ConsoleCommandController {
                        name: unsafe { sdk::get_event_console_command_name(event) }.to_string(),
                        args: unsafe { sdk::get_event_console_command_args(event) }
                            .iter()
                            .map(|s| s.to_string())
                            .collect(),
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
            unsafe {
                sdk::toggle_event_type(sdk_type as u16, true);
            }
        }

        let handlers = self.public_handlers.get_mut(&public_type);

        if let Some(handlers) = handlers {
            handlers.push(event);
        } else {
            self.public_handlers.insert(public_type, vec![event]);
        }
    }
}
