use std::{cell::RefCell, path::PathBuf, rc::Rc};

use altv_sdk::ffi as sdk;
use libloading::Library;
use resource_impl::resource_impl::{ResourceImpl, ResourceImplRef};
use resource_manager::ResourceController;

use crate::resource_manager::RESOURCE_MANAGER_INSTANCE;

mod helpers;
mod resource_manager;

type ResourceMainFn =
    unsafe extern "C" fn(core: *mut sdk::alt::ICore, resource_impl: ResourceImplRef);

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(full_main_path: &str) {
    let full_main_path = full_main_path.to_string();
    logger::error!("resource_start: {full_main_path}");

    let core_ptr = unsafe { sdk::get_alt_core() };

    let resource_impl = Rc::new(RefCell::new(ResourceImpl::new()));

    let lib = unsafe { Library::new(PathBuf::from(&full_main_path)) }.unwrap();
    let main_fn: ResourceMainFn = unsafe { *lib.get(b"main\0").unwrap() };
    unsafe { main_fn(core_ptr, Rc::clone(&resource_impl)) };

    let resource_controller = ResourceController::new(lib, resource_impl);

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
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
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_resource_destroy_impl() {
    // logger::debug!("runtime_resource_destroy_impl");
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_on_tick() {
    RESOURCE_MANAGER_INSTANCE.with(|v| {
        for (_, controller) in v.borrow().resources_iter() {
            controller.borrow_resource_impl().__on_tick();
        }
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_event(full_main_path: &str, event: *const sdk::alt::CEvent) {
    let full_main_path = full_main_path.to_string();

    if event.is_null() {
        panic!("resource_on_event event is null");
    }

    let raw_type = unsafe { sdk::get_event_type(event) };
    logger::warn!("resource_on_event {raw_type:?}");

    let event_type = altv_sdk::EventType::from(raw_type).unwrap();

    logger::debug!(
        "resource_on_event full_main_path: {}, event: {:?}",
        full_main_path,
        event_type
    );

    // // heron said it will be removed
    if event_type == altv_sdk::EventType::PLAYER_BEFORE_CONNECT {
        logger::info!("ignoring PLAYER_BEFORE_CONNECT");
        return;
    }

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        let manager = manager.borrow();
        manager
            .get_by_path(&full_main_path)
            .unwrap_or_else(|| {
                panic!("[resource_on_event] failed to get resource by path: {full_main_path}");
            })
            .borrow_resource_impl()
            .__on_sdk_event(event_type, event);
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_create_base_object(
    full_main_path: &str,
    base_object: *mut sdk::alt::IBaseObject,
) {
    let full_main_path = full_main_path.to_string();

    if base_object.is_null() {
        panic!("resource_on_create_base_object base_object is null");
    }

    let base_object_type = helpers::get_base_object_type(base_object);

    logger::debug!("resource_on_create_base_object type: {base_object_type:?}",);

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        let manager = manager.borrow();
        manager
            .get_by_path(&full_main_path)
            .unwrap_or_else(|| {
                panic!("[resource_on_remove_base_object] failed to get resource by path: {full_main_path}");
            })
            .borrow_resource_impl()
            .__on_base_object_create(base_object, base_object_type);
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_remove_base_object(
    full_main_path: &str,
    base_object: *mut sdk::alt::IBaseObject,
) {
    let full_main_path = full_main_path.to_string();
    if base_object.is_null() {
        panic!("resource_on_remove_base_object base_object is null");
    }

    let base_object_type = helpers::get_base_object_type(base_object);

    logger::debug!(
        "resource_on_remove_base_object type: {:?}",
        base_object_type
    );

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        let manager = manager.borrow();
        manager
            .get_by_path(&full_main_path)
            .unwrap_or_else(|| {
                panic!("[resource_on_remove_base_object] failed to get resource by path: {full_main_path}");
            })
            .borrow_resource_impl()
            .__on_base_object_destroy(base_object);
    });
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn altMain(core: *mut sdk::alt::ICore) -> bool {
    std::env::set_var("RUST_BACKTRACE", "1");

    if core.is_null() {
        panic!("altMain core is null");
    }

    logger::init().unwrap();

    logger::info!("set_alt_core");
    sdk::set_alt_core(core as *mut sdk::alt::ICore);

    logger::info!("create_script_runtime");
    let runtime = sdk::create_script_runtime();

    logger::info!("register_script_runtime");
    sdk::register_script_runtime(core as *mut sdk::alt::ICore, "rs", runtime);

    logger::info!("setup_callbacks");
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
