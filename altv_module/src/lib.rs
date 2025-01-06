use altv_sdk::{ffi as sdk, ALT_SDK_VERSION};
use core_module::{CStringResourceName, CBool};
use helpers::current_thread_id;
use libloading::Library;
use schedule_start::{avoid_self_resource_start, AvoidEvent, ResourceSchedule, ScheduleStart};
use std::{
  ffi::{c_char, CString},
  path::PathBuf,
  ptr::NonNull,
};

use crate::{event_manager::EVENT_MANAGER_INSTANCE, resource_manager::RESOURCE_MANAGER_INSTANCE};

mod event_manager;
mod helpers;
mod required_sdk_events;
mod resource_manager;

// temp workaround for weird behavior added recently in alt:V core:
// resources are starting in different threads and after that are called from main thread
mod schedule_start;

#[allow(improper_ctypes_definitions)]
type ResourceMainFn = unsafe extern "C" fn(
  altv_module_version: CString, // should always be FIRST arg for backward compatibility!!!
  core: *mut sdk::alt::ICore,
  resource_name: CStringResourceName,
  resource_handlers: &mut core_module::ResourceHandlers,
  module_handlers: core_module::ModuleHandlers,
) -> CBool;

const ALTV_MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(resource_name: &str, full_main_path: &str) {
  let full_main_path = full_main_path.to_string();
  let resource_name = resource_name.to_string();
  logger::debug!("resource_start: {resource_name} ({full_main_path})");

  let lib = unsafe { Library::new(PathBuf::from(&full_main_path)) }.unwrap_or_else(|e| {
    panic!("Failed to load resource: {resource_name} from: {full_main_path}, reason: {e:#?}");
  });

  let main_fn: ResourceMainFn = unsafe { *lib.get(b"main\0").unwrap() };

  ScheduleStart::add(
    resource_name,
    ResourceSchedule {
      lib,
      main_fn,
      thread_id: current_thread_id(),
    },
  );
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

fn toggle_resource_event_type(
  resource_name: CStringResourceName,
  event_type: altv_sdk::EventType,
  state: bool,
) {
  logger::debug!(
    "toggle_resource_event_type {event_type:?} {state:?} (resource: {})",
    resource_name.to_str().unwrap()
  );

  EVENT_MANAGER_INSTANCE.with(|v| {
    v.borrow_mut()
      .toggle_event(resource_name.into_string().unwrap(), event_type, state);
  })
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_resource_destroy_impl() {
  // logger::debug!("runtime_resource_destroy_impl");
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_on_tick() {
  ScheduleStart::start_all();

  RESOURCE_MANAGER_INSTANCE.with(|v| {
    for (_, controller) in v.borrow().resources_iter() {
      controller.resource_for_module.on_tick();
    }
  });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_event(resource_name: &str, event: altv_sdk::CEventPtr) {
  let resource_name = resource_name.to_string();

  ScheduleStart::start_if_not_already(resource_name.clone());

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

  if let AvoidEvent::Yes = avoid_self_resource_start(&resource_name, event_type) {
    logger::debug!("avoiding self resource start");
    return;
  }

  logger::debug!(
    "resource_on_event resource_name: {}, event: {:?}",
    resource_name,
    event_type
  );

  RESOURCE_MANAGER_INSTANCE.with(|manager| {
    let manager = manager.borrow();
    manager
      .get_resource_for_module_by_name(&resource_name)
      .unwrap_or_else(|| {
        panic!("[resource_on_event] failed to get resource: {resource_name}");
      })
      .on_sdk_event(event_type, event);
  });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_create_base_object(
  resource_name: &str,
  base_object: altv_sdk::BaseObjectRawMutPtr,
) {
  let resource_name = resource_name.to_string();

  ScheduleStart::start_if_not_already(resource_name.clone());

  on_base_object_event!(
    on_base_object_create,
    &resource_name,
    NonNull::new(base_object).unwrap()
  );
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_remove_base_object(
  resource_name: &str,
  base_object: altv_sdk::BaseObjectRawMutPtr,
) {
  let resource_name = resource_name.to_string();

  ScheduleStart::start_if_not_already(resource_name.clone());

  on_base_object_event!(
    on_base_object_destroy,
    &resource_name,
    NonNull::new(base_object).unwrap()
  );
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn altMain(core: *mut sdk::alt::ICore) -> bool {
  if core.is_null() {
    panic!("altMain core is null");
  }

  logger::init().unwrap();

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
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn GetSDKHash() -> *const c_char {
  ALT_SDK_VERSION.as_ptr().cast()
}
