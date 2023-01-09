mod resource_manager;

use altv_sdk::ffi as sdk;
use cxx::let_cxx_string;
use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use once_cell::sync::OnceCell;
use resource_api::{timers, Managers, ResourceApi};
use std::{
    cell::RefCell,
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_create_impl(resource_impl: *mut sdk::RustResourceImpl) {
    println!("runtime_create_impl resource_impl: {:?}", resource_impl);
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_destroy_impl() {
    println!("runtime_destroy_impl");
}

#[derive(WrapperApi)]
struct ResourceDll {
    // core, full_main_path parameters are injected using alt::res_main macro
    main: unsafe extern "C" fn(
        core: *mut sdk::ICore,
        full_main_path: PathBuf,
        resource_api: Arc<Mutex<ResourceApi>>,
    ) -> alt::MainResource,
}

// static mut RESOURCE_MANAGER: Mutex<ResourceManager> =
//     Mutex::new(ResourceManager { resources: vec![] });

// static mut RESOURCE_TEST: Option<Rc<alt::MainResource>> = None;

thread_local! {
    static RESOURCES: RefCell<HashMap<PathBuf, alt::MainResource>> = RefCell::new(HashMap::new());
}

static MANAGERS_INSTANCE: OnceCell<Arc<Mutex<Managers>>> = OnceCell::new();
static RESOURCE_API: OnceCell<Arc<Mutex<ResourceApi>>> = OnceCell::new();

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(resource_path: &str, resource_main: &str) {
    let full_main_path = std::path::Path::new(resource_path).join(resource_main);
    dbg!(&full_main_path);

    let container: Container<ResourceDll> = unsafe {
        Container::load(&full_main_path).unwrap_or_else(|_| {
            panic!(
                "Failed to open rust resource dll at: {}",
                full_main_path.display()
            )
        })
    };

    let core_ptr = unsafe { sdk::alt_core_instance() };
    println!("calling resource main func with core ptr: {:?}", core_ptr);
    let resource = unsafe {
        container.main(
            core_ptr,
            full_main_path.clone(),
            Arc::clone(RESOURCE_API.get().expect("RESOURCE_API.get() failed")),
        )
    };

    // let resource_rc = Rc::new(resource);

    // unsafe {
    //     RESOURCE_MANAGER
    //         .try_lock()
    //         .unwrap()
    //         .add(full_main_path, resource_rc.clone());
    // }

    // unsafe {
    //     RESOURCE_TEST = Some(resource_rc);
    // }

    RESOURCES.with(|resources| {
        let mut resources = resources.borrow_mut();
        resources.insert(full_main_path, resource);
    });
}

extern "C" fn resource_on_tick() {
    // println!("resource_on_tick");

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

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn altMain(core: *mut sdk::ICore) -> bool {
    std::env::set_var("RUST_BACKTRACE", "1");

    sdk::set_alt_core(core);

    let runtime = sdk::create_script_runtime();
    let_cxx_string!(resource_type = "rs");

    sdk::register_script_runtime(
        core,
        &resource_type,
        runtime,
        altv_sdk::RuntimeCreateImplCallback(runtime_create_impl),
        altv_sdk::RuntimeDestroyImplCallback(runtime_destroy_impl),
        altv_sdk::ResourceStartCallback(resource_start),
        altv_sdk::ResourceOnTickCallback(resource_on_tick),
    );

    MANAGERS_INSTANCE.set(Arc::new(Mutex::new(Managers {
        timer_manager: timers::TimerManager::instance(),
    })));

    RESOURCE_API.set(Arc::new(Mutex::new(ResourceApi {
        managers: Arc::clone(MANAGERS_INSTANCE.get().unwrap()),
    })));

    true
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn GetSDKHash() -> *const std::ffi::c_char {
    std::ffi::CStr::from_bytes_with_nul(altv_sdk::ALT_SDK_VERSION)
        .unwrap()
        .as_ptr()
}
