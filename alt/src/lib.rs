use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use altv_sdk::ffi;
use cxx::let_cxx_string;

// internal shit
pub use ffi::set_alt_core as __set_alt_core;
pub use ffi::ICore as __alt_ICore;
pub use resource_api as __resource_api;

use once_cell::sync::OnceCell;
pub use resource_main_macro::resource_main_func as res_main;

pub type ModelHash = u32;

pub mod events;
pub mod player;

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

pub fn log(str: &str) {
    let_cxx_string!(cxx_str = str);
    unsafe {
        ffi::log_colored(&cxx_str);
    }
}

pub fn log_error(str: &str) {
    let_cxx_string!(cxx_str = str);
    unsafe {
        ffi::log_error(&cxx_str);
    }
}

pub fn log_warn(str: &str) {
    let_cxx_string!(cxx_str = str);
    unsafe {
        ffi::log_warn(&cxx_str);
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log(&format!($($arg)*))
    }}
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        $crate::log_error(&format!($($arg)*))
    }}
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        $crate::log_warn(&format!($($arg)*))
    }}
}

pub struct Vehicle {
    ptr: *mut ffi::IVehicle,
    id: u16,
}

impl Vehicle {
    pub fn all() -> Vec<RefCell<Vehicle>> {
        todo!()
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn destroy(self) {
        // TODO: baseobject validation shit
        unsafe { ffi::destroy_baseobject(ffi::convert_vehicle_to_baseobject(self.ptr)) }
    }
}

pub fn create_vehicle(
    model: ModelHash,
    x: f32,
    y: f32,
    z: f32,
    rx: f32,
    ry: f32,
    rz: f32,
) -> Option<RefCell<Vehicle>> {
    let ptr = unsafe { ffi::create_vehicle(model, x, y, z, rx, ry, rz) };
    if ptr.is_null() {
        return None;
    }

    let id = unsafe { ffi::get_entity_id(ffi::convert_vehicle_to_entity(ptr)) };

    Some(RefCell::new(Vehicle { id, ptr }))
}

pub struct MainResource {
    pub path: PathBuf,
}

static RESOURCE_API: OnceCell<Arc<Mutex<resource_api::ResourceApi>>> = OnceCell::new();

impl MainResource {
    pub fn new(path: PathBuf, resource_api: Arc<Mutex<resource_api::ResourceApi>>) -> Self {
        RESOURCE_API
            .set(resource_api)
            .unwrap_or_else(|_| panic!("RESOURCE_API.set failed"));

        MainResource { path }
    }

    pub fn on_tick(&mut self) {
        println!("MainResource on_tick path: {}", self.path.display());
    }
}

pub fn set_interval(callback: impl FnMut() + 'static, millis: u64) {
    RESOURCE_API
        .get()
        .unwrap()
        .try_lock()
        .unwrap()
        .create_timer(Box::new(callback), millis, false);
}

pub fn set_timeout(callback: impl FnMut() + 'static, millis: u64) {
    RESOURCE_API
        .get()
        .unwrap()
        .try_lock()
        .unwrap()
        .create_timer(Box::new(callback), millis, true);
}
