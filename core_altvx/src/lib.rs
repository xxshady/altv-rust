pub use core_shared::*;

pub use anyhow;
type VoidResult = anyhow::Result<()>;
type SomeResult<V> = anyhow::Result<V>;

use altv_sdk::ffi as sdk;

mod resource;
use resource::Resource;

mod base_object;
pub mod col_shape;
pub mod events;
mod helpers;
pub mod logging;
pub mod mvalue;
mod player;
pub mod script_events;
pub mod timers;
pub mod vector;
mod vehicle;
mod world_object;

pub use base_object::col_shape::ColShape;
pub use base_object::col_shape::ColShapeContainer;
pub use base_object::extra_pools::Entity;
pub use base_object::extra_pools::EntityId;
pub use base_object::player::Player;
pub use base_object::player::PlayerContainer;
pub use base_object::vehicle::Vehicle;
pub use base_object::vehicle::VehicleContainer;
pub use base_object::ValidBaseObject;
pub use world_object::WorldObject;

pub use helpers::hash;

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
}
