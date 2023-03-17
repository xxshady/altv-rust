pub use core_shared::ResourceHandlers;

pub use anyhow;

mod resource;
use resource::Resource;

pub mod base_object;
mod base_object_maps;
pub mod client_events;
pub mod entity;
pub mod events;
mod helpers;
pub mod logging;
pub mod mvalue;
pub mod player;
pub mod script_events;
pub mod timers;
pub mod vector;
pub mod vehicle;

pub fn init(resource_handlers: &mut ResourceHandlers) {
    logger::init().unwrap();
    logger::debug!("init");

    Resource::init();

    macro_rules! set_callback {
        ($name: ident, $closure: expr) => {
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

    set_callback!(on_base_object_destroy, |base_object| {
        Resource::with(|resource| {
            resource.on_base_object_destroy(base_object);
        });
    });
}
