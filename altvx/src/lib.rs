pub use resource_main_macro::resource_main_func as main;
// __internal is intended for resource_main_func proc macro ^
#[doc(hidden)]
pub mod __internal {
    pub use altv_sdk::ffi::{alt::ICore, set_alt_core};

    pub use core_altvx::{init as core_init, ResourceHandlers};

    pub fn init(resource_state: &mut ResourceHandlers) {
        core_init(resource_state)
    }
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

pub use core_altvx::player::Player;
pub use core_altvx::vehicle::Vehicle;

// credits to altv-rs creator
// https://github.com/justdimaa/altv-rs/blob/f5cf1733493466634793804dfb1ca6d387fbe687/altv-sdk/src/lib.rs#L24
pub fn hash(str: &str) -> u32 {
    let bytes = str.as_bytes();
    let mut num: std::num::Wrapping<u32> = std::num::Wrapping(0u32);

    for n in bytes {
        num += std::num::Wrapping(*n as u32);
        num += num << 10;
        num ^= num >> 6;
    }

    num += num << 3;
    num ^= num >> 11;

    (num + (num << 15)).0
}

pub fn set_timeout(callback: impl FnMut() + 'static, millis: u64) {
    core_altvx::timers::create_timer(Box::new(callback), millis, true);
}

pub fn set_interval(callback: impl FnMut() + 'static, millis: u64) {
    core_altvx::timers::create_timer(Box::new(callback), millis, false);
}
