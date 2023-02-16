use cxx::{type_id, ExternType};
use extern_type_callback_derive::ExternTypeCallback;

// this file should be generated automatically later if not already created
mod cpp_sdk_version;
pub use cpp_sdk_version::ALT_SDK_VERSION;

mod event_type;
pub use event_type::EventType;

mod base_object_type;
pub use base_object_type::BaseObjectType;

// TODO: maybe change this proc macro to macro-by-example
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
pub struct ResourceOnTickCallback(pub extern "C" fn(full_main_path: &str));

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct ResourceOnEventCallback(
    pub extern "C" fn(full_main_path: &str, event: *const ffi::CEvent),
);

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct ResourceOnCreateBaseObjectCallback(
    pub extern "C" fn(full_main_path: &str, base_object: *mut ffi::IBaseObject),
);

#[derive(ExternTypeCallback)]
#[repr(transparent)]
pub struct ResourceOnRemoveBaseObjectCallback(
    pub extern "C" fn(full_main_path: &str, base_object: *mut ffi::IBaseObject),
);

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
        type ResourceOnCreateBaseObjectCallback = crate::ResourceOnCreateBaseObjectCallback;
        type ResourceOnRemoveBaseObjectCallback = crate::ResourceOnRemoveBaseObjectCallback;

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

        #[allow(clippy::too_many_arguments)]
        unsafe fn setup_callbacks(
            create_impl: RuntimeCreateImplCallback,
            destroy_impl: RuntimeDestroyImplCallback,
            on_tick_callback: RuntimeOnTickCallback,
            resource_start: ResourceStartCallback,
            resource_stop: ResourceStopCallback,
            resource_on_tick_callback: ResourceOnTickCallback,
            resource_on_event_callback: ResourceOnEventCallback,
            resource_on_create_base_object_callback: ResourceOnCreateBaseObjectCallback,
            resource_on_remove_base_object_callback: ResourceOnRemoveBaseObjectCallback,
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

        // entity conversions
        unsafe fn convert_base_object_to_entity(base_object: *mut IBaseObject) -> *mut IEntity;

        // vehicle conversions
        unsafe fn convert_vehicle_to_base_object(base_object: *mut IVehicle) -> *mut IBaseObject;
        unsafe fn convert_base_object_to_vehicle(vehicle: *mut IBaseObject) -> *mut IVehicle;
        unsafe fn convert_vehicle_to_entity(entity: *mut IVehicle) -> *mut IEntity;

        // player conversions
        unsafe fn convert_player_to_base_object(base_object: *mut IPlayer) -> *mut IBaseObject;
        unsafe fn convert_base_object_to_player(player: *mut IBaseObject) -> *mut IPlayer;
        unsafe fn convert_player_to_entity(entity: *mut IPlayer) -> *mut IEntity;

        // base_object
        unsafe fn destroy_base_object(base_object: *mut IBaseObject);
        unsafe fn get_base_object_type(base_object: *const IBaseObject) -> u8;

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
        unsafe fn set_vehicle_primary_color(vehicle: *mut IVehicle, color: u8);
        unsafe fn get_vehicle_primary_color(vehicle: *const IVehicle) -> u8;

        // player
        unsafe fn get_player_name(player: *mut IPlayer) -> UniquePtr<CxxString>;
    }
}
