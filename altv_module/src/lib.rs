use autocxx::prelude::*;
use altv_sdk::ffi as sdk;
use std::ffi::CString;
use crate::{
    resource_manager::{RESOURCE_MANAGER_INSTANCE, ResourceController},
    helpers::{handle_base_object_creation_or_deletion},
};

mod resource_manager;
mod wasi;
mod const_asserts;
mod helpers;
mod wasi_stdout_err;

type ResourceName = String;

const ALTV_MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(
    resource_name: &str,
    full_main_path: &str,
    resource_impl: *mut sdk::shared::AltResourceImpl,
    resource_ptr: *mut sdk::shared::AltResource,
) -> bool {
    let full_main_path = full_main_path.to_string();
    let resource_name = resource_name.to_string();

    logger::debug!("resource_start: {resource_name} ({full_main_path})");

    let mut exist = false;
    let content = unsafe { sdk::read_file(resource_impl, &full_main_path, &mut exist) };
    if !exist {
        logger::error!("Failed to start resource: {resource_name}, main file: '{full_main_path}' does not exist");
        return false;
    }

    logger::debug!("resource main file content len: {}", content.len());

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        let res = ResourceController::new(resource_name.clone(), resource_ptr, content.as_slice());

        let mut manager_mut = manager.borrow_mut();

        let mut success = false;
        match res {
            Ok(controller) => {
                success = true;
                manager_mut.add(resource_name.clone(), controller);
            }
            Err(e) => {
                logger::error!("Failed to start resource: {resource_name}, error: {e:?}");
            }
        }

        drop(manager_mut);

        if success {
            let manager_ref = manager.borrow();
            let controller = manager_ref.get_by_name(&resource_name).unwrap();
            let res = controller.call_main();

            if let Err(err) = res {
                logger::error!("Failed to start resource: {resource_name}, error: {err:?}");
                controller.log_error_message();

                // not needed because resource will be removed later in resource_stop
                // drop(manager_ref);
                // manager.borrow_mut().remove(&resource_name);

                success = false;
            }
        }

        manager.borrow_mut().remove_pending_status(&resource_name);

        success
    })
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
extern "C" fn runtime_resource_impl_create(resource_name: &str) {
    logger::debug!("runtime_resource_impl_create resource: {resource_name}");

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        manager
            .borrow_mut()
            .add_pending_status(resource_name.to_string());
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_tick(resource_name: &str) {
    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        let mut manager = manager.borrow_mut();

        let Some(controller) = manager.get_by_name(resource_name) else {
            logger::error!("resource_on_tick unknown resource: {resource_name}");
            return;
        };

        let res = controller.call_export(|e| e.call_on_tick());
        if res.is_err() {
            manager.resource_panicked(resource_name.to_string());
        }
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_event(_resource_name: &str, _event: altv_sdk::CEventPtr) {}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_create_base_object(
    resource_name: &str,
    base_object: altv_sdk::BaseObjectRawMutPtr,
) {
    logger::debug!(
        "resource_on_create_base_object resource: {resource_name:?} object: {base_object:?}"
    );
    handle_base_object_creation_or_deletion(resource_name, base_object, true);
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_remove_base_object(
    resource_name: &str,
    base_object: altv_sdk::BaseObjectRawMutPtr,
) {
    logger::debug!(
        "resource_on_remove_base_object resource: {resource_name:?} object: {base_object:?}"
    );
    handle_base_object_creation_or_deletion(resource_name, base_object, false);
}

#[no_mangle]
fn main(core: usize, runtime: usize) {
    logger::init(|message| altv_sdk::helpers::log(&message)).unwrap();

    unsafe {
        sdk::set_alt_core(core as *mut _);
        sdk::set_runtime(runtime as *mut _);
        logger::debug!("after set_alt_core & set_runtime");

        logger::debug!("setup_callbacks");
        sdk::setup_callbacks(
            sdk::ResourceStartCallback(resource_start),
            sdk::ResourceStopCallback(resource_stop),
            sdk::RuntimeResourceDestroyImplCallback(runtime_resource_destroy_impl),
            sdk::RuntimeResourceImplCreateCallback(runtime_resource_impl_create),
            sdk::ResourceOnTickCallback(resource_on_tick),
            sdk::ResourceOnEventCallback(resource_on_event),
            sdk::ResourceOnCreateBaseObjectCallback(resource_on_create_base_object),
            sdk::ResourceOnRemoveBaseObjectCallback(resource_on_remove_base_object),
        );

        logger::debug!("natives::init");
        sdk::natives::init();
    }
}
