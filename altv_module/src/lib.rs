use altv_sdk::ffi as sdk;
// use core_module::{result::VoidResult, ResourceName};
// TEMP
type ResourceName = String;

use autocxx::{WithinUniquePtr, prelude::UniquePtr};
use std::{path::PathBuf, ptr::NonNull, ffi::CString, cell::RefCell};

use crate::resource_manager::{RESOURCE_MANAGER_INSTANCE, ResourceController};

// use crate::{event_manager::EVENT_MANAGER_INSTANCE, resource_manager::RESOURCE_MANAGER_INSTANCE};

// mod event_manager;
mod helpers;
mod required_sdk_events;
mod resource_manager;
mod wasi;
mod types;

const ALTV_MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(
    resource_name: &str,
    full_main_path: &str,
    resource_impl: *mut sdk::shared::AltResourceImpl,
    resource_ptr: *mut sdk::shared::AltResource,
) {
    let full_main_path = full_main_path.to_string();
    let resource_name = resource_name.to_string();

    logger::debug!("resource_start: {resource_name} ({full_main_path})");

    let mut exist = false;
    let content = unsafe { sdk::read_file(resource_impl, &full_main_path, &mut exist) };
    if !exist {
        logger::error!("Failed to start resource: {resource_name}, main file: '{full_main_path}' does not exist");
        return;
    }

    logger::debug!("resource main file content len: {}", content.len());

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        manager
            .borrow_mut()
            .add_pending_status(resource_name.clone());

        let res = wasi::start(content.as_slice());
        logger::debug!("start res: {res:?}");

        let mut manager = manager.borrow_mut();

        manager.remove_pending_status(&resource_name);

        match res {
            Ok((instance, stdout_reader)) => {
                manager.add(
                    resource_name,
                    ResourceController::new(instance, stdout_reader, resource_ptr),
                );
            }
            Err(e) => {
                logger::error!("Failed to start resource: {resource_name}, error: {e:?}");
            }
        }
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_stop(resource_name: &str) {
    let resource_name = resource_name.to_string();
    logger::debug!("resource_stop: {resource_name}");

    // RESOURCE_MANAGER_INSTANCE.with(|manager| {
    //     manager.borrow_mut().remove(&resource_name);
    // });
    // EVENT_MANAGER_INSTANCE.with(|manager| {
    //     manager.borrow_mut().resource_stopped(&resource_name);
    // });
}

fn toggle_resource_event_type(
    resource_name: ResourceName,
    event_type: altv_sdk::EventType,
    state: bool,
) {
    logger::debug!(
        "toggle_resource_event_type {event_type:?} {state:?} (resource: {resource_name})"
    );

    // EVENT_MANAGER_INSTANCE.with(|v| {
    //     v.borrow_mut()
    //         .toggle_event(resource_name, event_type, state);
    // })
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_resource_destroy_impl() {
    // logger::debug!("runtime_resource_destroy_impl");
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_on_tick() {
    RESOURCE_MANAGER_INSTANCE.with(|v| {
        for (_, controller) in v.borrow().resources_iter() {
            let line = controller.read_stdout_line();
            if line.is_empty() {
                continue;
            }

            let line_without_break = line.get(..(line.len() - 1));
            let Some(line_without_break) = line_without_break else { continue; };

            unsafe {
                altv_sdk::helpers::log_with_resource(line_without_break, controller.ptr);
            }
        }
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_event(resource_name: &str, event: altv_sdk::CEventPtr) {
    // let resource_name = resource_name.to_string();

    // if event.is_null() {
    //     panic!("resource_on_event event is null");
    // }

    // let raw_type = unsafe { sdk::CEvent::GetType(event) };
    // let event_type = altv_sdk::EventType::try_from(raw_type).unwrap();

    // if let altv_sdk::EventType::CreateBaseObjectEvent | altv_sdk::EventType::RemoveBaseObjectEvent =
    //     event_type
    // {
    //     logger::debug!("ignoring create/remove baseobject event");
    //     return;
    // }

    // logger::debug!(
    //     "resource_on_event resource_name: {}, event: {:?}",
    //     resource_name,
    //     event_type
    // );

    // RESOURCE_MANAGER_INSTANCE.with(|manager| {
    //     let manager = manager.borrow();
    //     manager
    //         .get_resource_for_module_by_name(&resource_name)
    //         .unwrap_or_else(|| {
    //             panic!("[resource_on_event] failed to get resource: {resource_name}");
    //         })
    //         .on_sdk_event(event_type, event);
    // });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_create_base_object(
    resource_name: &str,
    base_object: altv_sdk::BaseObjectRawMutPtr,
) {
    // let resource_name = resource_name.to_string();
    // on_base_object_event!(
    //     on_base_object_create,
    //     &resource_name,
    //     NonNull::new(base_object).unwrap()
    // );
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_remove_base_object(
    resource_name: &str,
    base_object: altv_sdk::BaseObjectRawMutPtr,
) {
    // let resource_name = resource_name.to_string();
    // on_base_object_event!(
    //     on_base_object_destroy,
    //     &resource_name,
    //     NonNull::new(base_object).unwrap()
    // );
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn CreateScriptRuntime(
    core: *mut sdk::alt::ICore,
) -> *mut sdk::alt::IScriptRuntime {
    if core.is_null() {
        panic!("CreateScriptRuntime core is null");
    }

    logger::init().unwrap();

    sdk::set_alt_core(core as *mut sdk::alt::ICore);

    logger::debug!("create_script_runtime");
    let runtime = sdk::create_script_runtime();

    logger::debug!("register_script_runtime");
    sdk::register_script_runtime(core as *mut sdk::alt::ICore, "rs", runtime);

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

    runtime
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn GetSDKHash() -> *const std::ffi::c_char {
    // TODO: use CString
    std::ffi::CStr::from_bytes_with_nul(altv_sdk::ALT_SDK_VERSION)
        .unwrap()
        .as_ptr()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn GetType() -> *const std::ffi::c_char {
    let string = CString::new("rs").expect("Failed to create string in GetType");
    string.into_raw()
}
