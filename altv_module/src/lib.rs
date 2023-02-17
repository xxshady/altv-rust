mod resource_manager;

use altv_sdk::ffi as sdk;
use cxx::let_cxx_string;
use libloading::Library;
use resource_impl::resource_impl::ResourceImplContainer;
use std::path::PathBuf;

use crate::resource_manager::RESOURCE_MANAGER_INSTANCE;

mod helpers;

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_create_impl(resource_impl: *mut sdk::RustResourceImpl) {
    resource_impl::log!("runtime_create_impl resource_impl: {:?}", resource_impl);
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_destroy_impl() {
    resource_impl::log!("runtime_destroy_impl");
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_on_tick() {
    // resource_impl::log!("runtime_on_tick");

    // let resources = ResourceManager::instance().resources_iter();

    RESOURCE_MANAGER_INSTANCE.with(|v| {
        for (_, resource) in v.borrow().resources_iter() {
            resource.with(|v| {
                v.borrow().__on_tick();
            })
        }
    });
}

type ResourceMainFn = unsafe extern "C" fn(
    core: *mut sdk::ICore,
    full_main_path: String,
    __on_resource_impl_create: fn(resource: ResourceImplContainer),
);

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(full_main_path: &str) {
    let full_main_path = full_main_path.to_string();
    dbg!(&full_main_path);

    let core_ptr = unsafe { sdk::alt_core_instance() };
    resource_impl::log!("calling resource main func with core ptr: {:?}", core_ptr);

    let lib = unsafe { Library::new(PathBuf::from(&full_main_path)) }.unwrap();
    let main_fn: ResourceMainFn = unsafe { *lib.get(b"main\0").unwrap() };
    unsafe { main_fn(core_ptr, full_main_path, __on_resource_impl_create) };

    Box::leak(Box::new(lib));
}

fn __on_resource_impl_create(resource: ResourceImplContainer) {
    resource_impl::log_warn!("on_resource_impl_create");

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        resource.with(|resource_cell| {
            manager
                .borrow_mut()
                .add(resource_cell.borrow().full_main_path.clone(), resource);
        })
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_stop(full_main_path: &str) {
    resource_impl::log!("resource stop callback");
    dbg!(&full_main_path);

    // TODO: some cleanup of timers, altv entities (?), etc.
}

extern "C" fn resource_on_tick(full_main_path: &str) {
    // resource_impl::log!("resource_on_tick");
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_event(full_main_path: &str, event: *const sdk::CEvent) {
    if event.is_null() {
        panic!("resource_on_event event is null");
    }

    let raw_type = unsafe { sdk::get_event_type(event) };
    let event_type = altv_sdk::EventType::from(raw_type).unwrap();

    resource_impl::log!(
        "resource_on_event full_main_path: {}, event: {:?}",
        full_main_path,
        event_type
    );

    // heron said it will be removed
    if event_type == altv_sdk::EventType::PLAYER_BEFORE_CONNECT {
        resource_impl::log_warn!("ignoring PLAYER_BEFORE_CONNECT");
        return;
    }

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        let manager = manager.borrow();
        manager
            .get_by_path(full_main_path)
            .unwrap_or_else(|| {
                panic!("[resource_on_event] failed to get resource by path: {full_main_path}");
            })
            .with(|res| {
                res.borrow().__on_sdk_event(event_type, event);
            });
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_create_base_object(
    full_main_path: &str,
    base_object: *mut sdk::IBaseObject,
) {
    if base_object.is_null() {
        panic!("resource_on_create_base_object base_object is null");
    }

    let base_object_type = helpers::get_base_object_type(base_object);

    resource_impl::log_warn!("resource_on_create_base_object type: {base_object_type:?}",);

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        let manager = manager.borrow();
        manager
            .get_by_path(full_main_path)
            .unwrap_or_else(|| {
                panic!("[resource_on_event] failed to get resource by path: {full_main_path}");
            })
            .with(|res| {
                res.borrow()
                    .__on_base_object_create(base_object, base_object_type)
            });
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_remove_base_object(
    full_main_path: &str,
    base_object: *mut sdk::IBaseObject,
) {
    if base_object.is_null() {
        panic!("resource_on_remove_base_object base_object is null");
    }

    let base_object_type = helpers::get_base_object_type(base_object);

    resource_impl::log_warn!(
        "resource_on_remove_base_object type: {:?}",
        base_object_type
    );

    RESOURCE_MANAGER_INSTANCE.with(|manager| {
        let manager = manager.borrow();
        manager
            .get_by_path(full_main_path)
            .unwrap_or_else(|| {
                panic!("[resource_on_event] failed to get resource by path: {full_main_path}");
            })
            .with(|res| res.borrow().__on_base_object_destroy(base_object));
    });
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn altMain(core: *mut sdk::ICore) -> bool {
    std::env::set_var("RUST_BACKTRACE", "1");

    if core.is_null() {
        panic!("altMain core is null");
    }

    sdk::set_alt_core(core);

    let runtime = sdk::create_script_runtime();
    let_cxx_string!(resource_type = "rs");

    sdk::register_script_runtime(core, &resource_type, runtime);

    sdk::setup_callbacks(
        altv_sdk::RuntimeCreateImplCallback(runtime_create_impl),
        altv_sdk::RuntimeDestroyImplCallback(runtime_destroy_impl),
        altv_sdk::RuntimeOnTickCallback(runtime_on_tick),
        altv_sdk::ResourceStartCallback(resource_start),
        altv_sdk::ResourceStopCallback(resource_stop),
        altv_sdk::ResourceOnTickCallback(resource_on_tick),
        altv_sdk::ResourceOnEventCallback(resource_on_event),
        altv_sdk::ResourceOnCreateBaseObjectCallback(resource_on_create_base_object),
        altv_sdk::ResourceOnRemoveBaseObjectCallback(resource_on_remove_base_object),
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
