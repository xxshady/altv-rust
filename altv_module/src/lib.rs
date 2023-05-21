use altv_sdk::ffi as sdk;
use core_module::{result::VoidResult, ResourceName};
use libloading::Library;
use resource_manager::ResourceController;
use std::{path::PathBuf, ptr::NonNull};

use crate::{event_manager::EVENT_MANAGER_INSTANCE, resource_manager::RESOURCE_MANAGER_INSTANCE};

mod event_manager;
mod helpers;
mod required_sdk_events;
mod resource_manager;

type ResourceMainFn = unsafe extern "C" fn(
    altv_module_version: String, // should always be FIRST arg for backward compatibility!!!
    core: *mut sdk::alt::ICore,
    resource_name: ResourceName,
    resource_handlers: &mut core_module::ResourceHandlers,
    module_handlers: core_module::ModuleHandlers,
) -> VoidResult;

const ALTV_MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(resource_name: &str, full_main_path: &str) {
    let full_main_path = full_main_path.to_string();
    let resource_name = resource_name.to_string();
    logger::debug!("resource_start: {resource_name} ({full_main_path})");

    let core_ptr = unsafe { sdk::get_alt_core() };

    let module_handlers = core_module::ModuleHandlers::new(toggle_resource_event_type);

    let resource_handlers = core_module::ResourceHandlers::default();
    let mut resource_for_module = core_module::ResourceForModule::new(resource_handlers);
    let lib = unsafe { Library::new(PathBuf::from(&full_main_path)) }.unwrap();
    let main_fn: ResourceMainFn = unsafe { *lib.get(b"main\0").unwrap() };

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        manager
            .borrow_mut()
            .add_pending_status(resource_name.clone());

        let result = unsafe {
            main_fn(
                ALTV_MODULE_VERSION.to_string(),
                core_ptr,
                resource_name.clone(),
                &mut resource_for_module.handlers,
                module_handlers,
            )
        };

        if let Err(err) = result {
            logger::error!("Resource: {resource_name:?} main function returned error: {err:?}");
            std::process::exit(0);
        }

        manager.borrow_mut().remove_pending_status(&resource_name);

        let resource_controller = ResourceController::new(lib, resource_for_module);

        manager.borrow_mut().add(resource_name, resource_controller);
    });
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
    resource_name: ResourceName,
    event_type: altv_sdk::EventType,
    state: bool,
) {
    logger::debug!(
        "toggle_resource_event_type {event_type:?} {state:?} (resource: {resource_name})"
    );

    EVENT_MANAGER_INSTANCE.with(|v| {
        v.borrow_mut()
            .toggle_event(resource_name, event_type, state);
    })
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_resource_destroy_impl() {
    // logger::debug!("runtime_resource_destroy_impl");
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_on_tick() {
    RESOURCE_MANAGER_INSTANCE.with(|v| {
        for (_, controller) in v.borrow().resources_iter() {
            controller.resource_for_module.on_tick();
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

    true
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn GetSDKHash() -> *const std::ffi::c_char {
    std::ffi::CStr::from_bytes_with_nul(altv_sdk::ALT_SDK_VERSION)
        .unwrap()
        .as_ptr()
}
