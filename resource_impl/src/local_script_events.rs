use std::{collections::HashMap, fmt::Debug};

use crate::mvalue::{self, convert_vec_to_mvalue_vec, Serializable};
use altv_sdk::ffi as sdk;

pub fn emit_local_event(event_name: &str, args: Vec<Serializable>) {
    unsafe { sdk::trigger_local_event(event_name, convert_vec_to_mvalue_vec(args)) };
}

pub fn emit_local_event_without_args(event_name: &str) {
    unsafe { sdk::trigger_local_event(event_name, sdk::create_mvalue_vec()) };
}

/// Example:
/// ```rust
/// altvx::events::emit!("example");
/// altvx::events::emit!("example", 1, 2, 3);
/// ```
#[macro_export]
macro_rules! emit_local_event {
    ($event_name: expr) => {{
        unsafe { $crate::local_script_events::emit_local_event_without_args($event_name) };
    }};
    ($event_name: expr, $($arg: expr),+ $(,)*) => {{
        unsafe { $crate::local_script_events::emit_local_event($event_name, vec![$($arg.into()),*]) };
    }};
}

#[macro_export]
macro_rules! mvalue_list {
    ($($arg: expr),+ $(,)*) => {
        vec![$($arg.into()),*]
    };
}

pub type LocalEventArgs<'a> = &'a mvalue::MValueList;
pub type LocalEventHandler = Box<dyn FnMut(LocalEventArgs) -> anyhow::Result<()>>;

pub(crate) struct LocalEventManager {
    handlers: HashMap<String, Vec<LocalEventHandler>>,
}

impl LocalEventManager {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn receive_event(&mut self, event_name: &str, args: LocalEventArgs) {
        let handlers = self.handlers.get_mut(event_name);
        if let Some(handlers) = handlers {
            for h in handlers {
                if let Err(error) = h(&args) {
                    crate::log_error!(
                        "local event handler of event: {event_name:?} failed with error: {error:?}"
                    )
                }
            }
        } else {
            // TEST
            crate::log_warn!(
                "LocalEventManager.receive_event no handlers for event: {event_name:?}"
            )
        }
    }

    pub fn add_handler(&mut self, event_name: String, handler: LocalEventHandler) {
        let handlers = self.handlers.get_mut(&event_name);

        if let Some(handlers) = handlers {
            handlers.push(handler);
        } else {
            self.handlers.insert(event_name, vec![handler]);
        }
    }
}

impl Debug for LocalEventManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalEventManager")
    }
}
