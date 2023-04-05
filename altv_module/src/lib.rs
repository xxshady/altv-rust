use altv_sdk::ffi as sdk;
use core_module::ResourceMainPath;
use libloading::Library;
use resource_manager::ResourceController;
use std::{path::PathBuf, ptr::NonNull};

use crate::{event_manager::EVENT_MANAGER_INSTANCE, resource_manager::RESOURCE_MANAGER_INSTANCE};

mod event_manager;
mod helpers;
mod resource_manager;

type ResourceMainFn = unsafe extern "C" fn(
    core: *mut sdk::alt::ICore,
    full_main_path: ResourceMainPath,
    resource_handlers: &mut core_module::ResourceHandlers,
    module_handlers: core_module::ModuleHandlers,
);

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(full_main_path: &str) {
    let full_main_path = full_main_path.to_string();
    logger::debug!("resource_start: {full_main_path}");

    let core_ptr = unsafe { sdk::get_alt_core() };

    let module_handlers = core_module::ModuleHandlers::new(toggle_resource_event_type);

    let resource_handlers = core_module::ResourceHandlers::default();
    let mut resource_for_module = core_module::ResourceForModule::new(resource_handlers);
    let lib = unsafe { Library::new(PathBuf::from(&full_main_path)) }.unwrap();
    let main_fn: ResourceMainFn = unsafe { *lib.get(b"main\0").unwrap() };

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        manager
            .borrow_mut()
            .add_pending_status(full_main_path.clone());

        unsafe {
            main_fn(
                core_ptr,
                full_main_path.clone(),
                &mut resource_for_module.handlers,
                module_handlers,
            )
        };

        manager.borrow_mut().remove_pending_status(&full_main_path);

        let resource_controller = ResourceController::new(lib, resource_for_module);

        manager
            .borrow_mut()
            .add(full_main_path, resource_controller);
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_stop(full_main_path: &str) {
    let full_main_path = full_main_path.to_string();
    logger::debug!("resource_stop: {full_main_path}");

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        manager.borrow_mut().remove(&full_main_path);
    });
    EVENT_MANAGER_INSTANCE.with(|manager| {
        manager.borrow_mut().resource_stopped(&full_main_path);
    });
}

fn toggle_resource_event_type(
    full_main_path: ResourceMainPath,
    event_type: altv_sdk::EventType,
    state: bool,
) {
    logger::debug!("toggle_resource_event_type {event_type:?} {state:?} (from: {full_main_path})");

    EVENT_MANAGER_INSTANCE.with(|v| {
        v.borrow_mut()
            .toggle_event(full_main_path, event_type, state);
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
extern "C" fn resource_on_event(full_main_path: &str, event: altv_sdk::CEventPtr) {
    let full_main_path = full_main_path.to_string();

    if event.is_null() {
        panic!("resource_on_event event is null");
    }

    let raw_type = unsafe { sdk::CEvent::GetType(event) };

    let event_type = altv_sdk::EventType::from(raw_type).unwrap();

    logger::debug!(
        "resource_on_event full_main_path: {}, event: {:?}",
        full_main_path,
        event_type
    );

    // heron said it will be removed
    if event_type == altv_sdk::EventType::PlayerBeforeConnect {
        logger::info!("ignoring PlayerBeforeConnect");
        return;
    }

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        let manager = manager.borrow();
        manager
            .get_resource_for_module_by_path(&full_main_path)
            .unwrap_or_else(|| {
                panic!("[resource_on_event] failed to get resource by path: {full_main_path}");
            })
            .on_sdk_event(event_type, event);
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_create_base_object(
    full_main_path: &str,
    base_object: altv_sdk::BaseObjectRawMutPtr,
) {
    let full_main_path = full_main_path.to_string();
    on_base_object_event!(
        on_base_object_create,
        &full_main_path,
        NonNull::new(base_object).unwrap()
    );
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_remove_base_object(
    full_main_path: &str,
    base_object: altv_sdk::BaseObjectRawMutPtr,
) {
    let full_main_path = full_main_path.to_string();
    on_base_object_event!(
        on_base_object_destroy,
        &full_main_path,
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

    true
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn GetSDKHash() -> *const std::ffi::c_char {
    std::ffi::CStr::from_bytes_with_nul(altv_sdk::ALT_SDK_VERSION)
        .unwrap()
        .as_ptr()
}
