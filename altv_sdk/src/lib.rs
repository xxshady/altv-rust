#[allow(clippy::missing_safety_doc)]

pub mod ffi {
    use autocxx::prelude::*;
    use cxx::{type_id, ExternType};
    use extern_type_callback_derive::ExternTypeCallback;

    include_cpp! {
        #include "alt_bridge.h"

        name!(alt_bridge)

        generate!("set_alt_core")
        generate!("get_alt_core")
        generate!("create_script_runtime")
        generate!("register_script_runtime")

        // mvalue
        generate!("MValueWrapper")
        generate!("convert_mvalue_mut_wrapper_to_const")
        generate!("create_mvalue_vec")
        generate!("push_to_mvalue_vec")
        generate!("get_mvalue_type")
        generate!("get_mvalue_bool")
        generate!("get_mvalue_double")
        generate!("get_mvalue_string")
        generate!("get_mvalue_int")
        generate!("get_mvalue_uint")
        generate!("get_mvalue_list")

        generate!("get_mvalue_dict")
        generate!("get_mvalue_dict_pair_key")
        generate!("get_mvalue_dict_pair_value")

        generate!("get_mvalue_base_object")

        generate!("create_mvalue_bool")
        generate!("create_mvalue_double")
        generate!("create_mvalue_string")
        generate!("create_mvalue_nil")
        generate!("create_mvalue_int")
        generate!("create_mvalue_uint")
        generate!("create_mvalue_list")
        generate!("create_mvalue_dict")
        generate!("push_to_mvalue_dict")
        generate!("create_mvalue_base_object")

        // events
        generate!("trigger_local_event")
        generate!("trigger_client_event")
        generate!("trigger_client_event_for_some")
        generate!("trigger_client_event_for_all")
        generate!("trigger_local_event")
        generate!("toggle_event_type")
        generate!("get_event_type")
        generate!("get_event_player_target")
        generate!("get_event_reason")
        generate!("get_event_console_command_name")
        generate!("get_event_console_command_args")
        generate!("get_event_server_script_event_name")
        generate!("get_event_server_script_event_args")

        // base_object conversions
        generate!("convert_base_object_to_entity")
        generate!("convert_player_to_entity")
        generate!("convert_base_object_to_player")
        generate!("convert_player_to_base_object")
        generate!("convert_vehicle_to_entity")
        generate!("convert_base_object_to_vehicle")
        generate!("convert_vehicle_to_base_object")

        // base_object
        generate!("get_base_object_type")
        generate!("destroy_base_object")

        // entity
        generate!("get_entity_id")

        // vehicle
        generate!("create_vehicle")
        generate!("get_vehicle_primary_color")
        generate!("set_vehicle_primary_color")

        // player
        generate!("get_player_name")

        // logging
        generate!("log_colored")
        generate!("log_warn")
        generate!("log_error")

        generate!("create_player_vec")
        generate!("push_to_player_vec")
    }
    pub use alt_bridge::*;

    #[derive(ExternTypeCallback)]
    #[repr(transparent)]
    pub struct ResourceStartCallback(pub extern "C" fn(full_main_path: &str));

    #[derive(ExternTypeCallback)]
    #[repr(transparent)]
    pub struct ResourceStopCallback(pub extern "C" fn(full_main_path: &str));

    #[derive(ExternTypeCallback)]
    #[repr(transparent)]
    pub struct RuntimeResourceDestroyImplCallback(pub extern "C" fn());

    #[derive(ExternTypeCallback)]
    #[repr(transparent)]
    pub struct RuntimeOnTickCallback(pub extern "C" fn());

    #[derive(ExternTypeCallback)]
    #[repr(transparent)]
    pub struct ResourceOnEventCallback(
        pub extern "C" fn(full_main_path: &str, event: *const alt_bridge::alt::CEvent),
    );

    #[derive(ExternTypeCallback)]
    #[repr(transparent)]
    pub struct ResourceOnCreateBaseObjectCallback(
        pub extern "C" fn(full_main_path: &str, base_object: *mut alt_bridge::alt::IBaseObject),
    );

    #[derive(ExternTypeCallback)]
    #[repr(transparent)]
    pub struct ResourceOnRemoveBaseObjectCallback(
        pub extern "C" fn(full_main_path: &str, base_object: *mut alt_bridge::alt::IBaseObject),
    );

    #[cxx::bridge(namespace = "callbacks")]
    mod callbacks {
        unsafe extern "C++" {
            include!("callbacks.h");

            type ResourceStartCallback = super::ResourceStartCallback;
            type ResourceStopCallback = super::ResourceStopCallback;
            type RuntimeResourceDestroyImplCallback = super::RuntimeResourceDestroyImplCallback;
            type RuntimeOnTickCallback = super::RuntimeOnTickCallback;
            type ResourceOnEventCallback = super::ResourceOnEventCallback;
            type ResourceOnCreateBaseObjectCallback = super::ResourceOnCreateBaseObjectCallback;
            type ResourceOnRemoveBaseObjectCallback = super::ResourceOnRemoveBaseObjectCallback;

            #[allow(clippy::too_many_arguments)]
            unsafe fn setup_callbacks(
                resource_start: ResourceStartCallback,
                resource_stop: ResourceStopCallback,
                resource_impl_destroy: RuntimeResourceDestroyImplCallback,
                on_tick: RuntimeOnTickCallback,
                resource_on_event: ResourceOnEventCallback,
                resource_on_create_base_object: ResourceOnCreateBaseObjectCallback,
                resource_on_remove_base_object: ResourceOnRemoveBaseObjectCallback,
            );
        }
    }
    pub use callbacks::*;
}

pub type CEventPtr = *const ffi::alt::CEvent;
pub type IBaseObjectMutPtr = *mut ffi::alt::IBaseObject;

// auto generated files:

mod cpp_sdk_version;
pub use cpp_sdk_version::ALT_SDK_VERSION;

mod base_object_type;
pub use base_object_type::BaseObjectType;

mod event_type;
pub use event_type::EventType;

mod mvalue_type;
pub use mvalue_type::MValueType;
