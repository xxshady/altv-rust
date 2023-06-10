#![allow(clippy::new_ret_no_self)]

use core_shared::*;

use altv_sdk::ffi as sdk;

mod resource;
use resource::Resource;
mod alt_resource;
mod base_object_funcs;
mod base_objects;
mod blip;
mod checkpoint;
mod client_events;
mod col_shape;
mod config_node;
mod connection_info;
mod core_funcs;
mod events;
mod helpers;
mod logging;
mod marker;
mod meta;
mod network_object;
mod ped;
mod player;
mod quaternion;
mod rgba;
mod script_events;
mod structs;
mod timers;
mod vector;
mod vehicle;
mod virtual_entities;
mod voice_channel;
mod vehicle_model_info;
mod ped_model_info;
mod weapon_model_info;

pub use result::{IntoVoidResult, SomeResult, VoidResult};

pub mod exports;

pub fn init(
    resource_name: ResourceName,
    resource_handlers: &mut ResourceHandlers,
    module_handlers: ModuleHandlers,
) {
    logger::init().unwrap();
    logger::debug!("init");

    Resource::init(resource_name, module_handlers);

    macro_rules! set_callback {
        ($name:ident, $closure:expr) => {
            paste::paste! {
                resource_handlers.[<$name>].replace(Box::new($closure));
            }
        };
    }

    set_callback!(on_tick, || {
        Resource::with_timers_mut(|mut timers, resource| {
            timers.process_timers(resource.timer_schedule.borrow_mut());
        });
    });

    set_callback!(on_sdk_event, |event_type, event| {
        Resource::with_events_mut(|mut events, resource| {
            events.on_sdk_event(event_type, event, resource);
        });
    });

    set_callback!(on_base_object_create, |base_object, base_object_type| {
        Resource::with(|resource| {
            resource.on_base_object_create(base_object, base_object_type);
        });
    });

    set_callback!(on_base_object_destroy, |base_object, base_object_type| {
        Resource::with(|resource| {
            resource.on_base_object_destroy(base_object, base_object_type);
        });
    });

    script_events::LocalEventManager::init();
    script_events::ClientEventManager::init();
}
