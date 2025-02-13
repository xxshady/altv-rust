#![allow(clippy::new_ret_no_self)]

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
mod object;
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
mod mvalue_hash_map;
mod any_mvalue;
pub mod exports;

#[cfg(feature = "clientside")]
mod clientside;
#[cfg(feature = "reloading")]
mod reloading;

use core_shared::{
  abi_stable::{OwnedStr, Str},
  exports::Exports,
  result,
};
pub use result::{IntoVoidResult, SomeResult, VoidResult};
use altv_sdk::{ffi as sdk, ffi::alt::ICore};

relib_interface::include_exports!();
use gen_exports::ModuleExportsImpl;
relib_interface::include_imports!();

impl Exports for ModuleExportsImpl {
  fn altv_crate_version() -> OwnedStr {
    env!("CARGO_PKG_VERSION").to_owned().into()
  }

  fn init(core_ptr: *mut ICore, resource_name: Str) {
    let resource_name = unsafe { resource_name.to_string() }.to_owned();

    logger::init().unwrap();
    logger::debug!("init");

    unsafe {
      sdk::set_alt_core(core_ptr);
    }
    Resource::init(resource_name);
    script_events::LocalEventManager::init();
    script_events::ClientEventManager::init();

    #[cfg(feature = "reloading")]
    crate::reloading::init();
  }

  fn on_base_object_create(base_object: altv_sdk::BaseObjectMutPtr, ty: altv_sdk::BaseObjectType) {
    Resource::with(|resource| {
      resource.on_base_object_create(base_object, ty);
    });
  }

  fn on_base_object_destroy(base_object: altv_sdk::BaseObjectMutPtr, ty: altv_sdk::BaseObjectType) {
    Resource::with(|resource| {
      resource.on_base_object_destroy(base_object, ty);
    });
  }

  fn on_sdk_event(sdk_event_type: altv_sdk::EventType, event: altv_sdk::CEventPtr) {
    Resource::with_events_mut(|mut events, resource| {
      events.on_sdk_event(sdk_event_type, event, resource);
    });
  }

  fn on_tick() {
    Resource::with_timers_mut(|mut timers, resource| {
      timers.process_timers(resource.timer_schedule.borrow_mut());
    });
  }
}
