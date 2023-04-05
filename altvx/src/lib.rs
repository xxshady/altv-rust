pub use resource_main_macro::resource_main_func as main;
// __internal is intended for resource_main_func proc macro ^
#[doc(hidden)]
pub mod __internal {
    pub use altv_sdk::ffi::{alt::ICore, set_alt_core};

    pub use core_altvx::{init as core_init, ModuleHandlers, ResourceHandlers};

    pub fn init(
        full_main_path: String,
        resource_state: &mut ResourceHandlers,
        module_handlers: ModuleHandlers,
    ) {
        core_init(full_main_path, resource_state, module_handlers);
    }
}

pub mod prelude {
    pub use core_altvx::Entity;
    pub use core_altvx::ValidBaseObject;
    pub use core_altvx::WorldObject;
}

pub mod events;

// macros
pub use core_altvx::log;
pub use core_altvx::log_error;
pub use core_altvx::log_warn;
// functions
pub use core_altvx::logging::log;
pub use core_altvx::logging::log_error;
pub use core_altvx::logging::log_warn;

pub use core_altvx::vector::Vector2;
pub use core_altvx::vector::Vector3;
pub use core_altvx::ColShape;
pub use core_altvx::EntityId;
pub use core_altvx::Player;
pub use core_altvx::Vehicle;

// TEST
pub use altv_sdk::ffi;

pub use anyhow;

pub use core_altvx::hash;

pub fn set_timeout(callback: impl FnMut() -> anyhow::Result<()> + 'static, millis: u64) {
    core_altvx::timers::create_timer(Box::new(callback), millis, true);
}

pub fn set_interval(callback: impl FnMut() -> anyhow::Result<()> + 'static, millis: u64) {
    core_altvx::timers::create_timer(Box::new(callback), millis, false);
}
