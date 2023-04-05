pub use core_shared::*;

pub use anyhow;
pub type VoidResult = anyhow::Result<()>;
pub type SomeResult<V> = anyhow::Result<V>;

use altv_sdk::ffi as sdk;

mod resource;
use resource::Resource;

mod base_objects;
pub mod client_events;
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
mod virtual_entities;
mod world_object;

pub use base_objects::col_shape::ColShape;
pub use base_objects::col_shape::ColShapeContainer;
pub use base_objects::extra_pools::Entity;
pub use base_objects::extra_pools::EntityId;
pub use base_objects::player::Player;
pub use base_objects::player::PlayerContainer;
pub use base_objects::vehicle::Vehicle;
pub use base_objects::vehicle::VehicleContainer;
pub use base_objects::virtual_entity::VirtualEntity;
pub use base_objects::virtual_entity::VirtualEntityContainer;
pub use base_objects::virtual_entity_group::VirtualEntityGroup;
pub use base_objects::virtual_entity_group::VirtualEntityGroupContainer;
pub use base_objects::ValidBaseObject;
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
    script_events::ClientEventManager::init();
}
