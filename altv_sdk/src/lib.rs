use cxx::{type_id, ExternType};
use extern_type_callback_derive::ExternTypeCallback;

// this file should be generated automatically later if not already created
mod cpp_sdk_version;
pub use cpp_sdk_version::ALT_SDK_VERSION;

mod event_type;
pub use event_type::EventType;

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct RuntimeCreateImplCallback(pub extern "C" fn(resource_impl: *mut ffi::RustResourceImpl));

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct RuntimeDestroyImplCallback(pub extern "C" fn());

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct RuntimeOnTickCallback(pub extern "C" fn());

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct ResourceStartCallback(pub extern "C" fn(full_main_path: &str));

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct ResourceStopCallback(pub extern "C" fn(full_main_path: &str));

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct ResourceOnTickCallback(pub extern "C" fn());

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct ResourceOnEventCallback(pub extern "C" fn(event: *const ffi::CEvent));

#[allow(clippy::missing_safety_doc)]
#[cxx::bridge(namespace = "alt_rs")]
pub mod ffi {
    unsafe extern "C++" {
        include!("altv_sdk/include/alt_bridge.h");

        type RuntimeCreateImplCallback = crate::RuntimeCreateImplCallback;
        type RuntimeDestroyImplCallback = crate::RuntimeDestroyImplCallback;
        type RuntimeOnTickCallback = crate::RuntimeOnTickCallback;
        type ResourceStartCallback = crate::ResourceStartCallback;
        type ResourceStopCallback = crate::ResourceStopCallback;
        type ResourceOnTickCallback = crate::ResourceOnTickCallback;
        type ResourceOnEventCallback = crate::ResourceOnEventCallback;
        type ICore;
        type IScriptRuntime;
        type IBaseObject;
        type IEntity;
        type IVehicle;
        type IPlayer;
        type RustResourceImpl;
        type CEvent;

        // internal shit
        unsafe fn set_alt_core(core: *mut ICore);
        unsafe fn alt_core_instance() -> *mut ICore;
        unsafe fn create_script_runtime() -> *mut IScriptRuntime;
        unsafe fn register_script_runtime(
            core: *mut ICore,
            resource_type: &CxxString,
            runtime: *mut IScriptRuntime,
        );
        unsafe fn setup_callbacks(
            create_impl: RuntimeCreateImplCallback,
            destroy_impl: RuntimeDestroyImplCallback,
            on_tick_callback: RuntimeOnTickCallback,
            resource_start: ResourceStartCallback,
            resource_stop: ResourceStopCallback,
            resource_on_tick_callback: ResourceOnTickCallback,
            resource_on_event_callback: ResourceOnEventCallback,
        );

        // events
        // TODO: fix event_type to enum type instead of u16
        unsafe fn toggle_event_type(event_type: u16, state: bool);
        // CEvent methods
        unsafe fn get_event_type(event: *const CEvent) -> u16;
        unsafe fn get_event_player_target(event: *const CEvent) -> *mut IPlayer;
        unsafe fn get_event_reason(event: *const CEvent) -> UniquePtr<CxxString>;

        // --------------- script api ---------------
        // logging
        unsafe fn log_colored(str: &CxxString);
        unsafe fn log_error(str: &CxxString);
        unsafe fn log_warn(str: &CxxString);

        // vehicle conversions
        unsafe fn convert_vehicle_to_baseobject(baseobject: *mut IVehicle) -> *mut IBaseObject;
        unsafe fn convert_vehicle_to_entity(entity: *mut IVehicle) -> *mut IEntity;

        // player conversions
        unsafe fn convert_player_to_baseobject(baseobject: *mut IPlayer) -> *mut IBaseObject;
        unsafe fn convert_player_to_entity(entity: *mut IPlayer) -> *mut IEntity;

        // baseobject
        unsafe fn destroy_baseobject(baseobject: *mut IBaseObject);

        // entity
        unsafe fn get_entity_id(entity: *mut IEntity) -> u16;

        // vehicle
        unsafe fn create_vehicle(
            model: u32,
            x: f32,
            y: f32,
            z: f32,
            rx: f32,
            ry: f32,
            rz: f32,
        ) -> *mut IVehicle;

        // player
        unsafe fn get_player_name(player: *mut IPlayer) -> UniquePtr<CxxString>;
    }
}
