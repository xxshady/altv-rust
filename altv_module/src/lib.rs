mod resource_manager;

use std::{path::PathBuf, sync::Mutex};

use altv_sdk::ffi as sdk;
use cxx::let_cxx_string;
use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use lazy_static::lazy_static;
use resource_manager::ResourceManager;

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
    // core parameter should be injected using alt::res_main macro
    main: unsafe extern "C" fn(core: *mut sdk::ICore, full_main_path: PathBuf) -> alt::MainResource,
}

lazy_static! {
    static ref RESOURCE_MANAGER: Mutex<ResourceManager> = Mutex::new(ResourceManager::new());
}

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
    let resource = unsafe { container.main(core_ptr, full_main_path.clone()) };
    RESOURCE_MANAGER
        .try_lock()
        .unwrap()
        .add(full_main_path, resource);
}

extern "C" fn resource_on_tick() {
    for res in RESOURCE_MANAGER.try_lock().unwrap().resources.values_mut() {
        res.on_tick();
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn altMain(core: *mut sdk::ICore) -> bool {
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

    true
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn GetSDKHash() -> *const std::ffi::c_char {
    std::ffi::CStr::from_bytes_with_nul(altv_sdk::ALT_SDK_VERSION)
        .unwrap()
        .as_ptr()
}
