use core_shared::*;

pub type VoidResult = anyhow::Result<()>;
pub type SomeResult<V> = anyhow::Result<V>;

use altv_sdk::ffi as sdk;

mod resource;
use resource::Resource;
mod base_objects;
mod client_events;
mod col_shape;
mod events;
mod helpers;
mod logging;
mod mvalue;
mod player;
mod script_events;
mod timers;
mod vector;
mod vehicle;
mod virtual_entities;
mod world_object;

pub mod exports;

pub fn init(
    full_main_path: ResourceMainPath,
    resource_handlers: &mut ResourceHandlers,
    module_handlers: ModuleHandlers,
) {
    logger::init().unwrap();
    logger::debug!("init");

    Resource::init(full_main_path, module_handlers);

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

    set_callback!(on_base_object_destroy, |base_object, base_object_type| {
        Resource::with(|resource| {
            resource.on_base_object_destroy(base_object, base_object_type);
        });
    });

    script_events::LocalEventManager::init();
    script_events::ClientEventManager::init();
}
