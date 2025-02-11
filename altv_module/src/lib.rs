#![allow(clippy::missing_safety_doc)]

use core_shared::{abi_stable::Str, imports::Imports};
use relib_host::Module as RelibModule;

use altv_sdk::{ffi as sdk, ALT_SDK_VERSION};
use resource_manager::ResourceManager;
use std::{ffi::c_char, ptr::NonNull};

use crate::{event_manager::EVENT_MANAGER_INSTANCE, resource_manager::RESOURCE_MANAGER_INSTANCE};

mod event_manager;
mod helpers;
mod required_sdk_events;
mod resource_manager;

const ALTV_MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

relib_interface::include_exports!();
relib_interface::include_imports!();
use gen_imports::ModuleImportsImpl;

impl Imports for ModuleImportsImpl {
  fn toggle_event_type(resource: Str, ty: altv_sdk::EventType, enable: bool) {
    let resource = unsafe { resource.to_string() };
    logger::debug!("toggle_event_type {ty:?} {enable:?} (resource: {resource})");

    EVENT_MANAGER_INSTANCE.with(|v| {
      v.borrow_mut().toggle_event(resource, ty, enable);
    })
  }
}

pub type Module = RelibModule<gen_exports::ModuleExports>;

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(resource_name: &str, full_main_path: &str) -> bool {
  let full_main_path = full_main_path.to_string();
  let resource_name = resource_name.to_string();

  logger::debug!("resource_start: {resource_name} ({full_main_path})");

  ResourceManager::start_resource(resource_name, full_main_path)
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_stop(resource_name: &str) {
  let resource_name = resource_name.to_string();
  logger::debug!("resource_stop: {resource_name}");

  RESOURCE_MANAGER_INSTANCE.with(|manager| {
    manager.borrow_mut().remove(&resource_name);
  });
  EVENT_MANAGER_INSTANCE.with(|manager| {
    manager.borrow_mut().resource_stopped(&resource_name);
  });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_resource_destroy_impl() {
  // logger::debug!("runtime_resource_destroy_impl");
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_on_tick() {
  RESOURCE_MANAGER_INSTANCE.with(|v| {
    for (_, controller) in v.borrow().resources_iter() {
      unsafe {
        // TODO: stop resource on panic if reloading is enabled
        controller.exports().on_tick().unwrap();
      }
    }
  });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_event(resource_name: &str, event: altv_sdk::CEventPtr) {
  let resource_name = resource_name.to_string();

  if event.is_null() {
    panic!("resource_on_event event is null");
  }

  let raw_type = unsafe { sdk::CEvent::GetType(event) };
  let event_type = altv_sdk::EventType::try_from(raw_type).unwrap();

  if let altv_sdk::EventType::CreateBaseObjectEvent | altv_sdk::EventType::RemoveBaseObjectEvent =
    event_type
  {
    logger::debug!("ignoring create/remove baseobject event");
    return;
  }

  logger::debug!(
    "resource_on_event resource_name: {}, event: {:?}",
    resource_name,
    event_type
  );

  RESOURCE_MANAGER_INSTANCE.with(|manager| {
    let manager = manager.borrow();
    unsafe {
      manager
        .get_resource_exports_by_name(&resource_name)
        .unwrap_or_else(|| {
          panic!("[resource_on_event] failed to get resource: {resource_name}");
        })
        .on_sdk_event(event_type, event)
        // TODO: stop resource on panic if reloading is enabled
        .unwrap();
    }
  });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_create_base_object(
  resource_name: &str,
  base_object: altv_sdk::BaseObjectRawMutPtr,
) {
  let resource_name = resource_name.to_string();

  unsafe {
    on_base_object_event!(
      on_base_object_create,
      &resource_name,
      NonNull::new(base_object).unwrap()
    );
  }
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_remove_base_object(
  resource_name: &str,
  base_object: altv_sdk::BaseObjectRawMutPtr,
) {
  let resource_name = resource_name.to_string();

  unsafe {
    on_base_object_event!(
      on_base_object_destroy,
      &resource_name,
      NonNull::new(base_object).unwrap()
    );
  }
}

#[no_mangle]
pub unsafe extern "C" fn altMain(core: *mut sdk::alt::ICore) -> bool {
  if core.is_null() {
    panic!("altMain core is null");
  }

  logger::init().unwrap();
  relib_host::forcibly_reinit_dbghelp();

  logger::debug!("set_alt_core");
  sdk::set_alt_core(core);

  logger::debug!("create_script_runtime");
  let runtime = sdk::create_script_runtime();

  logger::debug!("register_script_runtime");
  sdk::register_script_runtime(core, "rs", runtime);

  logger::debug!("setup_callbacks");
  sdk::setup_callbacks(
    sdk::ResourceStartCallback(resource_start),
    sdk::ResourceStopCallback(resource_stop),
    sdk::RuntimeResourceDestroyImplCallback(runtime_resource_destroy_impl),
    sdk::RuntimeOnTickCallback(runtime_on_tick),
    sdk::ResourceOnEventCallback(resource_on_event),
    sdk::ResourceOnCreateBaseObjectCallback(resource_on_create_base_object),
    sdk::ResourceOnRemoveBaseObjectCallback(resource_on_remove_base_object),
  );

  required_sdk_events::enable();

  logger::info!("{ALTV_MODULE_VERSION}");

  true
}

#[no_mangle]
pub unsafe extern "C" fn GetSDKHash() -> *const c_char {
  ALT_SDK_VERSION.as_ptr().cast()
}
