use crate::{
    helpers::get_player_from_event, mvalue, player::PlayerContainer, resource::Resource,
    script_events::ScriptEventManager,
};
use altv_sdk::ffi as sdk;
pub use altv_sdk::EventType as SDKEventType;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum PublicEventType {
    ServerStarted,
    PlayerConnect,
    PlayerDisconnect,
    ResourceStop,
    BaseObjectCreate,
    ConsoleCommand,
    ServerScriptEvent,
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
            SERVER_SCRIPT_EVENT => Self::ServerScriptEvent,
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

#[derive(Debug, Default)]
pub struct EventManager {
    public_handlers: HashMap<PublicEventType, Vec<Event>>,
    enabled_sdk_events: HashSet<SDKEventType>,
}

impl EventManager {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn on_sdk_event(
        &mut self,
        event_type: SDKEventType,
        event: *const sdk::alt::CEvent,
        resource: &Resource,
    ) {
        logger::debug!("received event: {:?}", event_type);

        // i'm just too retarded to make generic function...
        macro_rules! prepare_script_event_if_handled {
            ($manager: expr) => {{
                let event_name = unsafe { sdk::get_any_script_event_name(event) };
                let event_name = event_name.as_ref().unwrap().to_string();
                if $manager.is_event_handled(&event_name) {
                    let args = unsafe { sdk::get_any_script_event_args(event) };
                    let deserialized_args = mvalue::deserialize_mvalue_args(args, resource);
                    Some((event_name, deserialized_args))
                } else {
                    logger::debug!("script event name: {event_name} ({event_type:?}) is unhandled");
                    None
                }
            }};
        }

        match event_type {
            SDKEventType::SERVER_SCRIPT_EVENT => {
                if let Some((event_name, deserialized_args)) =
                    prepare_script_event_if_handled!(resource.local_script_events.borrow())
                {
                    resource
                        .local_script_events
                        .borrow_mut()
                        .handle_event(&event_name, &deserialized_args);
                }
            }
            SDKEventType::CLIENT_SCRIPT_EVENT => {
                if let Some((event_name, deserialized_args)) =
                    prepare_script_event_if_handled!(resource.client_script_events.borrow())
                {
                    resource.client_script_events.borrow_mut().handle_event(
                        &event_name,
                        get_player_from_event(event, resource),
                        &deserialized_args,
                    );
                }
            }
            _ => {
                let handlers = self.public_handlers.get_mut(&event_type.into());

                if let Some(handlers) = handlers {
                    for h in handlers {
                        use Event::*;
                        match h {
                            ServerStarted(callback) => callback(ServerStartedController {}),
                            PlayerConnect(callback) => callback(PlayerConnectController {
                                player: get_player_from_event(event, resource),
                            }),
                            PlayerDisconnect(callback) => callback(PlayerDisconnectController {
                                player: get_player_from_event(event, resource),
                                reason: unsafe { sdk::get_event_reason(event) }.to_string(),
                            }),
                            ConsoleCommand(callback) => callback(ConsoleCommandController {
                                name: unsafe { sdk::get_event_console_command_name(event) }
                                    .to_string(),
                                args: unsafe { sdk::get_event_console_command_args(event) }
                                    .iter()
                                    .map(|s| s.to_string())
                                    .collect(),
                            }),
                            _ => todo!(),
                        }
                    }
                } else {
                    logger::debug!(
                        "[on_sdk_event] received event without handlers: {:?}",
                        event_type
                    );
                }
            }
        }
    }

    pub fn add_handler(
        &mut self,
        public_type: PublicEventType,
        sdk_type: SDKEventType,
        event: Event,
    ) {
        logger::debug!("events add_handler event public_type: {public_type:?}");

        if sdk_type != SDKEventType::NONE && self.enabled_sdk_events.get(&sdk_type).is_none() {
            logger::debug!("sdk_type: {sdk_type:?} | enabling it in sdk");

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

pub fn add_handler(public_type: PublicEventType, sdk_type: altv_sdk::EventType, event: Event) {
    Resource::with_events_mut(|mut events, _| {
        events.add_handler(public_type, sdk_type, event);
    });
}
