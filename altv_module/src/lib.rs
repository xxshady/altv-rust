mod resource_manager;

use altv_sdk::ffi as sdk;
use cxx::let_cxx_string;
use libloading::Library;
use once_cell::sync::OnceCell;
use resource_api::{Managers, ResourceApi};
use std::{
    cell::RefCell,
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_create_impl(resource_impl: *mut sdk::RustResourceImpl) {
    alt::log!("runtime_create_impl resource_impl: {:?}", resource_impl);
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_destroy_impl() {
    alt::log!("runtime_destroy_impl");
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_on_tick() {
    // alt::log!("runtime_on_tick");

    MANAGERS_INSTANCE
        .get()
        .unwrap()
        .try_lock()
        .unwrap()
        .timer_manager
        .try_lock()
        .unwrap()
        .process_timers();
}

type ResourceMainFn = unsafe extern "C" fn(
    core: *mut sdk::ICore,
    full_main_path: PathBuf,
    resource_api: Arc<Mutex<ResourceApi>>,
) -> alt::MainResource;

thread_local! {
    static RESOURCES: RefCell<HashMap<PathBuf, alt::MainResource>> = RefCell::new(HashMap::new());
}

static MANAGERS_INSTANCE: OnceCell<Arc<Mutex<Managers>>> = OnceCell::new();
static RESOURCE_API: OnceCell<Arc<Mutex<ResourceApi>>> = OnceCell::new();

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(resource_path: &str, resource_main: &str) {
    let full_main_path = std::path::Path::new(resource_path).join(resource_main);
    dbg!(&full_main_path);

    let core_ptr = unsafe { sdk::alt_core_instance() };
    alt::log!("calling resource main func with core ptr: {:?}", core_ptr);

    let lib = unsafe { Library::new(full_main_path.clone()) }.unwrap();
    let main_fn: ResourceMainFn = unsafe { *lib.get(b"main\0").unwrap() };
    let resource = unsafe {
        main_fn(
            core_ptr,
            full_main_path.clone(),
            Arc::clone(RESOURCE_API.get().expect("RESOURCE_API.get() failed")),
        )
    };

    Box::leak(Box::new(lib));

    RESOURCES.with(|resources| {
        let mut resources = resources.borrow_mut();
        resources.insert(full_main_path, resource);
    });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_stop(resource_path: &str, resource_main: &str) {
    let full_main_path = std::path::Path::new(resource_path).join(resource_main);
    alt::log!("resource stop callback");
    dbg!(&full_main_path);

    // TODO: some cleanup of timers, altv entities (?), etc.
}

extern "C" fn resource_on_tick() {
    // alt::log!("resource_on_tick");
}

extern "C" fn resource_on_event(event: *const sdk::CEvent) {
    let raw_type = unsafe { sdk::get_event_type(event) };
    let event_type = altv_sdk::EventType::from(raw_type).unwrap();

    let event_manager = MANAGERS_INSTANCE
        .get()
        .unwrap()
        .try_lock()
        .unwrap()
        .event_manager
        .try_lock()
        .unwrap();

    if let Some(handlers) = event_manager.get_handlers().get(&event_type) {
        for handler in handlers {
            use resource_api::events::SDKEvent::*;
            match handler {
                ServerStarted(callback) => callback(),
                PlayerConnect(callback) => {
                    callback(unsafe { sdk::get_event_player_target(event) } as usize)
                }
                PlayerDisconnect(callback) => callback(
                    unsafe { sdk::get_event_player_target(event) } as usize,
                    unsafe { sdk::get_event_reason(event).to_string() },
                ),
            }
        }
    } else {
        alt::log_warn!("event type received: {:?} | without handlers", event_type)
    }
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

    MANAGERS_INSTANCE
        .set(Arc::new(Mutex::new(Managers {
            timer_manager: resource_api::timers::TimerManager::instance(),
            event_manager: resource_api::events::SDKEventManager::instance(),
        })))
        .unwrap_or_else(|_| panic!("MANAGERS_INSTANCE.set failed"));

    RESOURCE_API
        .set(Arc::new(Mutex::new(ResourceApi {
            managers: Arc::clone(MANAGERS_INSTANCE.get().unwrap()),
        })))
        .unwrap_or_else(|_| panic!("MANAGERS_INSTANCE.set failed"));

    true
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn GetSDKHash() -> *const std::ffi::c_char {
    std::ffi::CStr::from_bytes_with_nul(altv_sdk::ALT_SDK_VERSION)
        .unwrap()
        .as_ptr()
}
