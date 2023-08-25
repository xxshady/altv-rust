use altv_sdk::ffi as sdk;
use std::ffi::CString;
use crate::resource_manager::{RESOURCE_MANAGER_INSTANCE, ResourceController};

mod resource_manager;
mod wasi;
mod const_asserts;

type ResourceName = String;

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
        let res = wasi::start(content.as_slice(), resource_ptr);

        let mut manager = manager.borrow_mut();

        manager.remove_pending_status(&resource_name);

        match res {
            Ok(wasi_exports) => {
                let resource = ResourceController::new(resource_ptr, wasi_exports);
                manager.add(resource_name, resource);
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

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        manager.borrow_mut().remove(&resource_name);
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_resource_destroy_impl() {}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_on_tick() {
    RESOURCE_MANAGER_INSTANCE.with(|v| {
        for (name, controller) in v.borrow().resources_iter() {
            let res = controller.wasi_exports.borrow_mut().call_on_tick();
            if let Err(err) = res {
                logger::error!("Resource: {name} on_tick handler failed with error: {err:?}");
            }
        }
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_event(resource_name: &str, event: altv_sdk::CEventPtr) {}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_create_base_object(
    resource_name: &str,
    base_object: altv_sdk::BaseObjectRawMutPtr,
) {
    RESOURCE_MANAGER_INSTANCE.with(|v| {
        let manager = v.borrow();
        let Some(r) = manager.get_by_name(resource_name) else {
            logger::error!("Unknown resource: {resource_name:?}");
            return;
        };

        let ty = unsafe { sdk::IBaseObject::GetType(base_object) };
        r.wasi_exports
            .borrow_mut()
            .call_on_base_object_create(base_object as u64, ty)
            .unwrap_or_else(|e| {
                logger::error!("call_on_base_object_create failed: {e:?}");
            });
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_remove_base_object(
    resource_name: &str,
    base_object: altv_sdk::BaseObjectRawMutPtr,
) {
    RESOURCE_MANAGER_INSTANCE.with(|v| {
        let manager = v.borrow();
        let Some(r) = manager.get_by_name(resource_name) else {
            logger::error!("Unknown resource: {resource_name:?}");
            return;
        };

        let ty = unsafe { sdk::IBaseObject::GetType(base_object) };
        r.wasi_exports
            .borrow_mut()
            .call_on_base_object_destroy(base_object as u64, ty)
            .unwrap_or_else(|e| {
                logger::error!("call_on_base_object_destroy failed: {e:?}");
            });
    });
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

    std::panic::set_hook(Box::new(|info| {
        logger::error!("panic: {info}");
    }));

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
