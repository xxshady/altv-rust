use std::cell::RefCell;
use std::thread::LocalKey;

use altv_sdk::ffi;

pub mod events;

pub use resource_impl::log;
pub use resource_impl::log_error;
pub use resource_impl::log_warn;
pub use resource_impl::logging::log;
pub use resource_impl::logging::log_error;
pub use resource_impl::logging::log_warn;
pub use resource_main_macro::resource_main_func as main;

#[doc(hidden)]
pub use ffi::set_alt_core as __set_alt_core;
#[doc(hidden)]
pub use ffi::ICore as __alt_ICore;

// intended for resource_main_macro
#[doc(hidden)]
pub use resource_impl::resource_impl::ResourceImpl as __ResourceImpl;

pub use resource_impl::entity::Entity;
pub use resource_impl::entity::EntityId;
pub use resource_impl::player::Player;
pub use resource_impl::vector::Vector3;
pub use resource_impl::vehicle::Vehicle;

pub type ModelHash = u32;

// credits to altv-rs creator
// https://github.com/justdimaa/altv-rs/blob/f5cf1733493466634793804dfb1ca6d387fbe687/altv-sdk/src/lib.rs#L24
pub fn hash(str: &str) -> ModelHash {
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

pub fn set_interval(callback: impl FnMut() + 'static + Send + Sync, millis: u64) {
    resource_impl::resource_impl::timers_create(Box::new(callback), millis, false);
}

pub fn set_timeout(callback: impl FnMut() + 'static + Send + Sync, millis: u64) {
    resource_impl::resource_impl::timers_create(Box::new(callback), millis, true);
}

#[doc(hidden)]
pub fn __init(full_main_path: String) -> &'static LocalKey<RefCell<__ResourceImpl>> {
    __ResourceImpl::init(full_main_path)
}
