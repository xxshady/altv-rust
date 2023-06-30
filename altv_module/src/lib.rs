use altv_sdk::ffi as sdk;
// use core_module::{result::VoidResult, ResourceName};
// TEMP
type ResourceName = String;

use autocxx::{WithinUniquePtr, prelude::UniquePtr};
use libloading::Library;
use wasmer::{FunctionType, Function, FunctionEnv, Type};
// use resource_manager::ResourceController;
use std::{path::PathBuf, ptr::NonNull, ffi::CString, cell::RefCell};

// use crate::{event_manager::EVENT_MANAGER_INSTANCE, resource_manager::RESOURCE_MANAGER_INSTANCE};

// mod event_manager;
mod helpers;
mod required_sdk_events;
// mod resource_manager;

// type ResourceMainFn = unsafe extern "C" fn(
//     altv_module_version: String, // should always be FIRST arg for backward compatibility!!!
//     core: *mut sdk::alt::ICore,
//     resource_name: ResourceName,
//     resource_handlers: &mut core_module::ResourceHandlers,
//     module_handlers: core_module::ModuleHandlers,
// ) -> VoidResult;

const ALTV_MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(resource_name: &str, full_main_path: &str) {
    let full_main_path = full_main_path.to_string();
    let resource_name = resource_name.to_string();
    log!("resource_start: {resource_name} ({full_main_path})");

    TEST_STORE.with(|v| {
        let mut v = v.borrow_mut();

        let marker = unsafe {
            sdk::ICore::CreateMarker(
                0,
                0.,
                0.,
                72.,
                255,
                0,
                0,
                255,
                false,
                0,
                std::ptr::null_mut(),
            )
        };
        v.marker.replace(marker);
    });

    // let core_ptr = unsafe { sdk::get_alt_core() };

    // let module_handlers = core_module::ModuleHandlers::new(toggle_resource_event_type);

    // let resource_handlers = core_module::ResourceHandlers::default();
    // let mut resource_for_module = core_module::ResourceForModule::new(resource_handlers);
    // let lib = unsafe { Library::new(PathBuf::from(&full_main_path)) }.unwrap();
    // let main_fn: ResourceMainFn = unsafe { *lib.get(b"main\0").unwrap() };

    // RESOURCE_MANAGER_INSTANCE.with(|manager| {
    //     manager
    //         .borrow_mut()
    //         .add_pending_status(resource_name.clone());

    //     let result = unsafe {
    //         main_fn(
    //             ALTV_MODULE_VERSION.to_string(),
    //             core_ptr,
    //             resource_name.clone(),
    //             &mut resource_for_module.handlers,
    //             module_handlers,
    //         )
    //     };

    //     if let Err(err) = result {
    //         logger::error!("Resource: {resource_name:?} main function returned error: {err:?}");
    //     }

    //     manager.borrow_mut().remove_pending_status(&resource_name);

    //     let resource_controller = ResourceController::new(lib, resource_for_module);

    //     manager.borrow_mut().add(resource_name, resource_controller);
    // });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_stop(resource_name: &str) {
    let resource_name = resource_name.to_string();
    logger::debug!("resource_stop: {resource_name}");

    // RESOURCE_MANAGER_INSTANCE.with(|manager| {
    //     manager.borrow_mut().remove(&resource_name);
    // });
    // EVENT_MANAGER_INSTANCE.with(|manager| {
    //     manager.borrow_mut().resource_stopped(&resource_name);
    // });
}

fn toggle_resource_event_type(
    resource_name: ResourceName,
    event_type: altv_sdk::EventType,
    state: bool,
) {
    logger::debug!(
        "toggle_resource_event_type {event_type:?} {state:?} (resource: {resource_name})"
    );

    // EVENT_MANAGER_INSTANCE.with(|v| {
    //     v.borrow_mut()
    //         .toggle_event(resource_name, event_type, state);
    // })
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_resource_destroy_impl() {
    // logger::debug!("runtime_resource_destroy_impl");
}

#[allow(improper_ctypes_definitions)]
extern "C" fn runtime_on_tick() {
    // let opt: Option<(u8, f32)> = TEST_STORE.with(|v| {
    //     let mut v = v.borrow_mut();
    //     match v.marker {
    //         None => None,
    //         Some(m) => {
    //             let (_, _, _, _a) =
    //                 read_cpp_rgba(unsafe { sdk::IMarker::GetColor(m) }.within_unique_ptr());

    //                 let (_, _, scale) =
    //                 read_cpp_vector3(unsafe { sdk::IMarker::GetScale(m) }.within_unique_ptr());

    //             Some((_a, scale))
    //         }
    //     }
    // });

    // let Some((a, scale)) = opt else {
    //     log!("waiting for marker");
    //     return;
    // };

    // WASM_FUNC.with(|v| {
    //     WASM_STORE.with(|store| {
    //         let result = v.borrow().as_ref().unwrap().call(
    //             store.borrow_mut().as_mut().unwrap(),
    //             &[wasmer::Value::I32(a as i32), wasmer::Value::F32(scale)],
    //         );
    //         // log!("call result: {result:?}");
    //     });
    // });
    // RESOURCE_MANAGER_INSTANCE.with(|v| {
    //     for (_, controller) in v.borrow().resources_iter() {
    //         controller.resource_for_module.on_tick();
    //     }
    // });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_event(resource_name: &str, event: altv_sdk::CEventPtr) {
    // let resource_name = resource_name.to_string();

    // if event.is_null() {
    //     panic!("resource_on_event event is null");
    // }

    // let raw_type = unsafe { sdk::CEvent::GetType(event) };
    // let event_type = altv_sdk::EventType::try_from(raw_type).unwrap();

    // if let altv_sdk::EventType::CreateBaseObjectEvent | altv_sdk::EventType::RemoveBaseObjectEvent =
    //     event_type
    // {
    //     logger::debug!("ignoring create/remove baseobject event");
    //     return;
    // }

    // logger::debug!(
    //     "resource_on_event resource_name: {}, event: {:?}",
    //     resource_name,
    //     event_type
    // );

    // RESOURCE_MANAGER_INSTANCE.with(|manager| {
    //     let manager = manager.borrow();
    //     manager
    //         .get_resource_for_module_by_name(&resource_name)
    //         .unwrap_or_else(|| {
    //             panic!("[resource_on_event] failed to get resource: {resource_name}");
    //         })
    //         .on_sdk_event(event_type, event);
    // });
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_create_base_object(
    resource_name: &str,
    base_object: altv_sdk::BaseObjectRawMutPtr,
) {
    // let resource_name = resource_name.to_string();
    // on_base_object_event!(
    //     on_base_object_create,
    //     &resource_name,
    //     NonNull::new(base_object).unwrap()
    // );
}

#[allow(improper_ctypes_definitions)]
extern "C" fn resource_on_remove_base_object(
    resource_name: &str,
    base_object: altv_sdk::BaseObjectRawMutPtr,
) {
    // let resource_name = resource_name.to_string();
    // on_base_object_event!(
    //     on_base_object_destroy,
    //     &resource_name,
    //     NonNull::new(base_object).unwrap()
    // );
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

    required_sdk_events::enable();

    logger::info!("{ALTV_MODULE_VERSION}");

    test_wasmer();

    runtime
}

pub struct TestStore {
    marker: Option<*mut sdk::alt::IMarker>,
}

thread_local! {
    pub static WASM_FUNC: RefCell<Option<wasmer::Function>> = RefCell::new(None);
    pub static WASM_INSTANCE: RefCell<Option<wasmer::Instance>> = RefCell::new(None);
    pub static WASM_STORE: RefCell<Option<wasmer::Store>> = RefCell::new(None);

    pub static TEST_STORE: RefCell<TestStore> = RefCell::new(TestStore { marker: None });
}

fn test_wasmer() {
    use wasmer::{Store, Module, Instance, Value, imports};

    log!("test_wasmer");

    static WASM: &[u8] =
        include_bytes!("../../target/wasm32-unknown-unknown/release/wasmer_test.wasm");

    let mut store = Store::default();

    log!("test_wasmer 1");
    let module = Module::new(&store, WASM).unwrap();

    log!("test_wasmer 3");
    let kek_sig = FunctionType::new(vec![Type::I32, Type::F32], vec![]);
    let kek = Function::new(&mut store, &kek_sig, |args| {
        // log!("getting u8...");
        let a = args[0].unwrap_i32();
        let scale = args[1].unwrap_f32();
        // log!("a: {a:?}");

        TEST_STORE.with(|v| {
            let mut v = v.borrow_mut();
            let m = v.marker.unwrap();

            unsafe {
                sdk::IMarker::SetColor(m, 255, 0, 0, a as u8);
            }

            unsafe {
                sdk::IMarker::SetScale(m, scale, scale, scale);
            }
        });

        Ok(vec![])
    });

    static mut start_time: Option<std::time::Instant> = None;

    let fibo_end_sig = FunctionType::new(vec![], vec![]);
    let fibo_end = Function::new(&mut store, &fibo_end_sig, |args| {
        log!("fibo_end elapsed: {:?}", unsafe {
            start_time.unwrap().elapsed()
        });
        Ok(vec![])
    });

    log!("test_wasmer 4");

    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {
        "env" => {
            "fibo_end" => fibo_end,
        }
    };

    log!("test_wasmer 5");
    let instance = Instance::new(&mut store, &module, &import_object);
    log!("instance result: {instance:?}");
    let instance = instance.unwrap();
    log!("test_wasmer 6");

    let main_func = instance.exports.get_function("main");
    log!("main_func: {main_func:?}");

    unsafe {
        start_time.replace(std::time::Instant::now());
    }

    let call_result = main_func.unwrap().call(&mut store, &[]);
    log!("call_result: {call_result:?}");

    // WASM_INSTANCE.with(|v| {
    //     let mut v = v.borrow_mut();
    //     v.replace(instance);

    //     let instance = v.as_mut().unwrap();
    //     let res = instance.exports.get_function("hello_wasm");
    //     log!("test_wasmer hello_wasm: {res:?}");

    //     let func = res.unwrap().clone();

    //     WASM_FUNC.with(|v| {
    //         v.borrow_mut().replace(func);
    //     });
    // });

    // WASM_STORE.with(|v| {
    //     v.borrow_mut().replace(store);
    // })
}

pub fn log(str: &str) {
    unsafe {
        sdk::ICore::LogColored(str, std::ptr::null_mut());
    }
}

#[macro_export]
macro_rules! __log {
    ($($arg:tt)*) => {{
        $crate::log(&format!($($arg)*))
    }}
}
pub use __log as log;

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

fn read_cpp_rgba(cpp_rgba: autocxx::prelude::UniquePtr<sdk::RGBAWrapper>) -> (u8, u8, u8, u8) {
    let mut r = 0u8;
    let mut g = 0u8;
    let mut b = 0u8;
    let mut a = 0u8;
    unsafe {
        sdk::read_rgba(
            cpp_rgba.as_ref().unwrap(),
            &mut r as *mut _,
            &mut g as *mut _,
            &mut b as *mut _,
            &mut a as *mut _,
        );
    }

    (r, g, b, a)
}

pub fn read_cpp_vector3(cpp_vector: UniquePtr<sdk::Vector3Wrapper>) -> (f32, f32, f32) {
    let mut out_x = 0f32;
    let mut out_y = 0f32;
    let mut out_z = 0f32;
    unsafe {
        sdk::read_vector3(
            cpp_vector.as_ref().unwrap(),
            &mut out_x as *mut f32,
            &mut out_y as *mut f32,
            &mut out_z as *mut f32,
        );
    }
    (out_x, out_y, out_z)
}
