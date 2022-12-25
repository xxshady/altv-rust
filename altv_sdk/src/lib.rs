use cxx::{type_id, ExternType};
use extern_type_callback_derive::ExternTypeCallback;

// this file should be generated automatically later if not already created
include!("./cpp-sdk-version.rs");

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct RuntimeCreateImplCallback(pub extern "C" fn(resource_impl: *mut ffi::RustResourceImpl));

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct RuntimeDestroyImplCallback(pub extern "C" fn());

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct ResourceStartCallback(pub extern "C" fn(resource_path: &str, resource_main: &str));

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct ResourceOnTickCallback(pub extern "C" fn());

#[allow(clippy::missing_safety_doc)]
#[cxx::bridge(namespace = "alt_rs")]
pub mod ffi {
    unsafe extern "C++" {
        include!("altv_sdk/include/SDK_extend.h");

        type RuntimeCreateImplCallback = crate::RuntimeCreateImplCallback;
        type RuntimeDestroyImplCallback = crate::RuntimeDestroyImplCallback;
        type ResourceStartCallback = crate::ResourceStartCallback;
        type ResourceOnTickCallback = crate::ResourceOnTickCallback;
        type ICore;
        type IScriptRuntime;
        type IVehicle;
        type RustResourceImpl;

        // internal shit
        unsafe fn set_alt_core(core: *mut ICore);
        unsafe fn alt_core_instance() -> *mut ICore;
        unsafe fn create_script_runtime() -> *mut IScriptRuntime;
        unsafe fn register_script_runtime(
            core: *mut ICore,
            resource_type: &CxxString,
            runtime: *mut IScriptRuntime,
            create_impl: RuntimeCreateImplCallback,
            destroy_impl: RuntimeDestroyImplCallback,
            resource_start: ResourceStartCallback,
            resource_on_tick_callback: ResourceOnTickCallback,
        );

        // script api
        unsafe fn log_colored(str: &CxxString);
        unsafe fn create_vehicle(
            model: u32,
            x: f32,
            y: f32,
            z: f32,
            rx: f32,
            ry: f32,
            rz: f32,
        ) -> *mut IVehicle;
        unsafe fn vehicle_get_id(vehicle: *mut IVehicle) -> u16;
        unsafe fn vehicle_destroy(vehicle: *mut IVehicle);
    }
}
