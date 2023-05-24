use std::{collections::HashMap, fmt::Debug};

use crate::{
    base_objects::player, helpers::IntoString, resource::Resource, IntoVoidResult, VoidResult,
};
use altv_sdk::ffi as sdk;

pub fn emit(event_name: impl IntoString, args: mvalue::DynMValueArgs) -> VoidResult {
    unsafe { sdk::trigger_local_event(event_name.into_string(), mvalue::serialize_args(args)?) };
    Ok(())
}

pub type EventArgs<'a> = &'a Vec<mvalue::ConstMValue>;

#[derive(Debug)]
pub struct LocalEventContext<'a> {
    pub args: EventArgs<'a>,
}

#[derive(Debug)]
pub struct ClientEventContext<'a> {
    pub player: player::PlayerContainer,
    pub args: EventArgs<'a>,
}

pub type LocalEventHandler = Box<dyn FnMut(&LocalEventContext) -> VoidResult>;
pub type ClientEventHandler = Box<dyn FnMut(&ClientEventContext) -> VoidResult>;

pub trait ScriptEventManager {
    type Handler;

    fn handlers(&self) -> &HashMap<String, Vec<Self::Handler>>;
    fn handlers_mut(&mut self) -> &mut HashMap<String, Vec<Self::Handler>>;

    fn is_event_handled(&self, event_name: &str) -> bool {
        self.handlers().contains_key(event_name)
    }

    fn add_handler(&mut self, event_name: String, handler: Self::Handler) {
        self.handlers_mut()
            .entry(event_name)
            .or_insert(vec![])
            .push(handler);
    }

    fn get_handlers_for_event(&mut self, event_name: &str) -> Option<&mut Vec<Self::Handler>> {
        self.handlers_mut().get_mut(event_name)
    }
}

#[derive(Default)]
pub struct LocalEventManager {
    handlers: HashMap<String, Vec<LocalEventHandler>>,
}

impl LocalEventManager {
    pub fn init() {
        use crate::events;

        events::add_sdk_handler(events::SDKHandler::ServerScriptEvent(Box::new(|c| {
            let events::sdk_contexts::ServerScriptEvent { name, args } = c;

            Resource::with_local_script_events_mut(|mut events, _| {
                if !events.is_event_handled(name) {
                    logger::debug!("local event is unhandled: {name}");
                    return;
                }
                events.handle_event(name, args);
            });

            Ok(())
        })));
    }

    fn handle_event(&mut self, event_name: &str, args: EventArgs) {
        if let Some(handlers) = self.get_handlers_for_event(event_name) {
            let context = LocalEventContext { args };
            for h in handlers {
                if let Err(error) = h(&context) {
                    logger::error!("handler of event: {event_name:?} failed with error: {error:?}");
                } else {
                    logger::debug!("handler of event: {event_name:?} called successfully");
                }
            }
        } else {
            logger::debug!("handle_event no handlers for local event: {event_name:?}")
        }
    }
}

impl ScriptEventManager for LocalEventManager {
    type Handler = LocalEventHandler;

    fn handlers(&self) -> &HashMap<String, Vec<Self::Handler>> {
        &self.handlers
    }

    fn handlers_mut(&mut self) -> &mut HashMap<String, Vec<Self::Handler>> {
        &mut self.handlers
    }
}

impl Debug for LocalEventManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalEventManager{{todo}}")
    }
}

#[derive(Default)]
pub struct ClientEventManager {
    handlers: HashMap<String, Vec<ClientEventHandler>>,
}

impl ClientEventManager {
    pub fn init() {
        use crate::events;

        events::add_sdk_handler(events::SDKHandler::ClientScriptEvent(Box::new(|c| {
            let events::sdk_contexts::ClientScriptEvent { name, player, args } = c;

            Resource::with_client_script_events_mut(|mut events, _| {
                if !events.is_event_handled(name) {
                    logger::debug!("client event is unhandled: {name}");
                    return;
                }
                events.handle_event(name, player.clone(), args);
            });

            Ok(())
        })));
    }

    pub fn handle_event(
        &mut self,
        event_name: &str,
        player: player::PlayerContainer,
        args: EventArgs,
    ) {
        if let Some(handlers) = self.get_handlers_for_event(event_name) {
            let context = ClientEventContext { player, args };
            for h in handlers {
                if let Err(error) = h(&context) {
                    logger::error!(
                        "handler of client event: {event_name:?} failed with error: {error:?}"
                    );
                } else {
                    logger::debug!("handler of client event: {event_name:?} called successfully");
                }
            }
        } else {
            logger::debug!("handle_event no handlers for client event: {event_name:?}")
        }
    }
}

impl ScriptEventManager for ClientEventManager {
    type Handler = ClientEventHandler;

    fn handlers(&self) -> &HashMap<String, Vec<Self::Handler>> {
        &self.handlers
    }

    fn handlers_mut(&mut self) -> &mut HashMap<String, Vec<Self::Handler>> {
        &mut self.handlers
    }
}

impl Debug for ClientEventManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientEventManager {{todo}} ")
    }
}

pub fn on<V: IntoVoidResult>(
    event_name: impl IntoString,
    mut handler: impl FnMut(&LocalEventContext) -> V + 'static,
) {
    Resource::with_local_script_events_mut(|mut local_events, _| {
        local_events.add_handler(
            event_name.into_string(),
            Box::new(move |c| handler(c).into_void_result()),
        )
    });
}

pub fn on_player<V: IntoVoidResult>(
    event_name: impl IntoString,
    mut handler: impl FnMut(&ClientEventContext) -> V + 'static,
) {
    Resource::with_client_script_events_mut(|mut client_events, _| {
        client_events.add_handler(
            event_name.into_string(),
            Box::new(move |c| handler(c).into_void_result()),
        )
    });
}
