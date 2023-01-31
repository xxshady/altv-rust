use altv_sdk::ffi;
use std::cell::RefCell;

pub use resource_impl::log;
pub use resource_impl::log_error;
pub use resource_impl::log_warn;
pub use resource_impl::logging::log;
pub use resource_impl::logging::log_error;
pub use resource_impl::logging::log_warn;
pub use resource_main_macro::resource_main_func as res_main;

#[doc(hidden)]
pub use ffi::set_alt_core as __set_alt_core;
#[doc(hidden)]
pub use ffi::ICore as __alt_ICore;

// intended for resource_main_macro
#[doc(hidden)]
pub use resource_impl::resource_impl::ResourceImpl as __ResourceImpl;

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

// TODO: move implement this shit in resource_impl!!!!!

// pub struct Vehicle {
//     ptr: *mut ffi::IVehicle,
//     id: u16,
// }

// impl Vehicle {
//     pub fn all() -> Vec<RefCell<Vehicle>> {
//         todo!()
//     }

//     pub fn id(&self) -> u16 {
//         self.id
//     }

//     pub fn destroy(self) {
//         // TODO: baseobject validation shit
//         unsafe {
//             ffi::destroy_baseobject(
//                 ffi::convert_vehicle_to_baseobject(self.ptr) as *mut ffi::IBaseObject
//             )
//         }
//     }
// }

pub fn create_vehicle(
    model: ModelHash,
    x: f32,
    y: f32,
    z: f32,
    rx: f32,
    ry: f32,
    rz: f32,
) -> Option<resource_impl::vehicle::VehicleContainer> {
    resource_impl::resource_impl::ResourceImpl::instance()
        .create_vehicle(model, x, y, z, rx, ry, rz)
}

pub fn set_interval(callback: impl FnMut() + 'static + Send + Sync, millis: u64) {
    resource_impl::resource_impl::ResourceImpl::instance().create_timer(
        Box::new(callback),
        millis,
        false,
    );
}

pub fn set_timeout(callback: impl FnMut() + 'static + Send + Sync, millis: u64) {
    resource_impl::resource_impl::ResourceImpl::instance().create_timer(
        Box::new(callback),
        millis,
        true,
    );
}

#[doc(hidden)]
pub fn __init(full_main_path: String) {
    log!("alt __init");
    __ResourceImpl::init(full_main_path);
}
