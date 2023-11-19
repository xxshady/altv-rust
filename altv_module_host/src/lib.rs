use std::{
    ffi::CString,
    cell::{Cell, RefCell},
    fs,
};

use cxx::let_cxx_string;
use altv_sdk::ffi as sdk;
use libloading::Library;

thread_local! {
    static CORE_PTR: Cell<usize> = Cell::default();
    static RUNTIME_PTR: Cell<usize> = Cell::default();
    static CURRENT_RUST_MODULE: RefCell<Option<Library>> = RefCell::new(None);
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn CreateScriptRuntime(
    core: *mut sdk::alt::ICore,
) -> *mut sdk::alt::IScriptRuntime {
    if core.is_null() {
        panic!("CreateScriptRuntime core is null");
    }

    logger::init(|message| altv_sdk::helpers::log(&message)).unwrap();

    std::panic::set_hook(Box::new(|info| {
        logger::error!("panic: {info}");
    }));

    sdk::set_alt_core(core);
    logger::debug!("after set_alt_core");

    let runtime = sdk::create_script_runtime();

    logger::debug!("register_script_runtime");
    sdk::register_script_runtime(core, "rs", runtime);

    CORE_PTR.set(core as usize);
    RUNTIME_PTR.set(runtime as usize);

    // logger::debug!("setup_callbacks");
    // sdk::setup_callbacks(
    //     sdk::ResourceStartCallback(resource_start),
    //     sdk::ResourceStopCallback(resource_stop),
    //     sdk::RuntimeResourceDestroyImplCallback(runtime_resource_destroy_impl),
    //     sdk::RuntimeResourceImplCreateCallback(runtime_resource_impl_create),
    //     sdk::ResourceOnTickCallback(resource_on_tick),
    //     sdk::ResourceOnEventCallback(resource_on_event),
    //     sdk::ResourceOnCreateBaseObjectCallback(resource_on_create_base_object),
    //     sdk::ResourceOnRemoveBaseObjectCallback(resource_on_remove_base_object),
    // );

    let_cxx_string!(res_command = "res");

    extern "C" fn handle_restart_command(args: &str) {
        // let args = args.into_iter().map(|v| v.to_string()).collect::<Vec<_>>();
        logger::debug!(
            "restart args: {:?}",
            args.trim_end().split(" ").collect::<Vec<_>>()
        );

        unsafe {
            load_module();
        }
    }

    sdk::setup_command(
        core as *mut _,
        &res_command,
        sdk::ConsoleCommandCallback(handle_restart_command),
    );

    load_module();

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

unsafe fn load_module() {
    CURRENT_RUST_MODULE.with_borrow_mut(|current_module| {
        logger::debug!("dropping previous module...");
        current_module.take();

        let host_path = process_path::get_dylib_path().unwrap();
        let altv_root = host_path.parent().unwrap().parent().unwrap();
        let module_path = altv_root.join("rust.dll");

        logger::debug!("copying new module from {module_path:?}...");
        let new_module_path = module_path.parent().unwrap().join("rust/rust.dll");
        logger::debug!("copying new module to {new_module_path:?}...");

        fs::create_dir_all(new_module_path.parent().unwrap()).unwrap();
        fs::copy(&module_path, &new_module_path).unwrap();

        logger::debug!("loading new module from {new_module_path:?}...");

        let rust_module = Library::new(new_module_path).unwrap();

        type MainFn = unsafe extern "C" fn(core: usize, runtime: usize);
        let main_fn: MainFn = *rust_module.get(b"main\0").unwrap();
        main_fn(CORE_PTR.get(), RUNTIME_PTR.get());

        current_module.replace(rust_module);
    });
}
