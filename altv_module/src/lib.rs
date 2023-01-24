mod resource_manager;

use altv_sdk::ffi as sdk;
use cxx::let_cxx_string;
use libloading::Library;
use once_cell::sync::OnceCell;
use resource_manager::{ResourceImplMutex, ResourceManager};
use std::{path::PathBuf, sync::Mutex};

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
    resource_impl::log!("runtime_on_tick");

    let mutex = RESOURCES
        .get()
        .expect("[runtime_on_tick] RESOURCES.get() failed")
        .try_lock()
        .expect("[runtime_on_tick] RESOURCES try_lock() failed");

    let resources = mutex.resources_iter();

    for resource in resources {
        resource
            .try_lock()
            .expect("resource.try_lock() failed")
            .__on_tick();
    }
}

type ResourceMainFn =
    unsafe extern "C" fn(core: *mut sdk::ICore, full_main_path: PathBuf) -> ResourceImplMutex;

static RESOURCES: OnceCell<Mutex<ResourceManager>> = OnceCell::new();

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(resource_path: &str, resource_main: &str) {
    let full_main_path = std::path::Path::new(resource_path).join(resource_main);
    dbg!(&full_main_path);

    let core_ptr = unsafe { sdk::alt_core_instance() };
    resource_impl::log!("calling resource main func with core ptr: {:?}", core_ptr);

    let lib = unsafe { Library::new(full_main_path.clone()) }.unwrap();
    let main_fn: ResourceMainFn = unsafe { *lib.get(b"main\0").unwrap() };
    let resource = unsafe { main_fn(core_ptr, full_main_path.clone()) };

    Box::leak(Box::new(lib));

    RESOURCES
        .get()
        .expect("RESOURCES.get_mut() failed")
        .try_lock()
        .expect("RESOURCES try_lock() failed")
        .add(full_main_path, resource);
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_stop(resource_path: &str, resource_main: &str) {
    let full_main_path = std::path::Path::new(resource_path).join(resource_main);
    resource_impl::log!("resource stop callback");
    dbg!(&full_main_path);

    // TODO: some cleanup of timers, altv entities (?), etc.
}

extern "C" fn resource_on_tick() {
    // resource_impl::log!("resource_on_tick");
}

extern "C" fn resource_on_event(event: *const sdk::CEvent) {
    let raw_type = unsafe { sdk::get_event_type(event) };
    let event_type = altv_sdk::EventType::from(raw_type).unwrap();

    // let event_manager = MANAGERS_INSTANCE
    //     .get()
    //     .unwrap()
    //     .try_lock()
    //     .unwrap()
    //     .event_manager
    //     .try_lock()
    //     .unwrap();

    // if let Some(handlers) = event_manager.get_handlers().get(&event_type) {
    //     for handler in handlers {
    //         use resource_impl::events::SDKEvent::*;
    //         match handler {
    //             ServerStarted(callback) => callback(),
    //             PlayerConnect(callback) => {
    //                 callback(unsafe { sdk::get_event_player_target(event) } as usize)
    //             }
    //             PlayerDisconnect(callback) => callback(
    //                 unsafe { sdk::get_event_player_target(event) } as usize,
    //                 unsafe { sdk::get_event_reason(event).to_string() },
    //             ),
    //         }
    //     }
    // } else {
    //     resource_impl::log_warn!("event type received: {:?} | without handlers", event_type)
    // }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn altMain(core: *mut sdk::ICore) -> bool {
    std::env::set_var("RUST_BACKTRACE", "1");

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
    );

    RESOURCES
        .set(Mutex::new(ResourceManager::new()))
        .expect("RESOURCES.set failed");

    true
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn GetSDKHash() -> *const std::ffi::c_char {
    std::ffi::CStr::from_bytes_with_nul(altv_sdk::ALT_SDK_VERSION)
        .unwrap()
        .as_ptr()
}
