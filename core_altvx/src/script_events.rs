use std::{collections::HashMap, fmt::Debug};

use crate::{
    mvalue::{self, convert_vec_to_mvalue_vec, Serializable},
    player,
    resource::Resource,
};
use altv_sdk::ffi as sdk;

pub fn emit_local_event(event_name: &str, args: Vec<Serializable>) {
    unsafe { sdk::trigger_local_event(event_name, convert_vec_to_mvalue_vec(args)) };
}

pub fn emit_local_event_without_args(event_name: &str) {
    unsafe { sdk::trigger_local_event(event_name, sdk::create_mvalue_vec()) };
}

/// Examples
///
/// ```rust
/// altvx::events::emit!("example").unwrap();
/// ```
///
/// Sending primitives
/// ```rust
/// altvx::events::emit!("example", 1, 2, 3).unwrap();
/// ```
///
/// Sending lists
/// ```rust
/// altvx::events::emit!("example", altvx::events::list![1, 2, 3]).unwrap();
/// ```
#[macro_export]
macro_rules! emit_local_event {
    ($event_name: expr) => {
        unsafe { $crate::script_events::emit_local_event_without_args($event_name) };
    };
    ($event_name: expr, $($arg: expr),+ $(,)*) => {
        (|| -> $crate::anyhow::Result<()> {
            let vec = $crate::mvalue_list!($( $arg ),+)?;
            $crate::script_events::emit_local_event(
                $event_name,
                vec
            );
            Ok(())
        })()
    };
}

pub type EventArgs<'a> = &'a mvalue::MValueList;
pub type LocalEventHandler = Box<dyn FnMut(EventArgs) -> anyhow::Result<()>>;
pub type ClientEventHandler =
    Box<dyn FnMut(player::PlayerContainer, EventArgs) -> anyhow::Result<()>>;

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
    pub fn handle_event(&mut self, event_name: &str, args: EventArgs) {
        if let Some(handlers) = self.get_handlers_for_event(event_name) {
            for h in handlers {
                if let Err(error) = h(&args) {
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
    pub fn handle_event(
        &mut self,
        event_name: &str,
        player: player::PlayerContainer,
        args: EventArgs,
    ) {
        if let Some(handlers) = self.get_handlers_for_event(event_name) {
            for h in handlers {
                if let Err(error) = h(player.clone(), &args) {
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
        write!(f, "LocalEventManager{{todo}}")
    }
}

pub fn add_local_handler(
    event_name: String,
    handler: impl FnMut(EventArgs) -> anyhow::Result<()> + 'static,
) {
    Resource::with_local_script_events_mut(|mut local_events, _| {
        local_events.add_handler(event_name, Box::new(handler))
    });
}

pub fn add_client_handler(
    event_name: String,
    handler: impl FnMut(player::PlayerContainer, EventArgs) -> anyhow::Result<()> + 'static,
) {
    Resource::with_client_script_events_mut(|mut client_events, _| {
        client_events.add_handler(event_name, Box::new(handler))
    });
}
