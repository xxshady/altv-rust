#![allow(clippy::missing_safety_doc)]

use std::ptr::NonNull;

pub mod ffi {
    use crate::helpers::impl_extern_type_callback;
    use autocxx::prelude::*;
    use cxx::{type_id, ExternType};

    include_cpp! {
        #include "alt_bridge.h"
        #include "alt_classes/ICore.h"
        #include "alt_classes/IBaseObject.h"
        // #include "alt_classes/IWorldObject.h"
        // #include "alt_classes/IEntity.h"
        // #include "alt_classes/IPlayer.h"
        // #include "alt_classes/IVehicle.h"
        // #include "alt_classes/IPed.h"
        // #include "alt_classes/INetworkObject.h"
        // #include "alt_classes/IColShape.h"
        // #include "alt_classes/IBlip.h"
        #include "alt_classes/IMarker.h"
        // #include "alt_classes/ICheckpoint.h"
        // #include "alt_classes/CEvent.h"
        // #include "alt_classes/IResource.h"
        // #include "alt_classes/IVoiceChannel.h"

        // #include "alt_classes/CConsoleCommandEvent.h"
        // #include "alt_classes/CServerScriptEvent.h"
        // #include "alt_classes/CClientScriptEvent.h"
        // #include "alt_classes/CPlayerConnectEvent.h"
        // #include "alt_classes/CPlayerDisconnectEvent.h"
        // #include "alt_classes/CColShapeEvent.h"
        // #include "alt_classes/IVirtualEntity.h"
        // #include "alt_classes/IVirtualEntityGroup.h"
        // #include "alt_classes/CWeaponDamageEvent.h"

        // entity
        // #include "alt_classes/CNetOwnerChangeEvent.h"

        // player
        // #include "alt_classes/CPlayerDeathEvent.h"
        // #include "alt_classes/CPlayerDamageEvent.h"
        // #include "alt_classes/CPlayerEnteringVehicleEvent.h"
        // #include "alt_classes/CPlayerEnterVehicleEvent.h"
        // #include "alt_classes/CPlayerLeaveVehicleEvent.h"
        // #include "alt_classes/CPlayerChangeAnimationEvent.h"
        // #include "alt_classes/CPlayerChangeVehicleSeatEvent.h"
        // #include "alt_classes/CPlayerWeaponChangeEvent.h"
        // #include "alt_classes/CPlayerConnectDeniedEvent.h"
        // #include "alt_classes/CPlayerSpawnEvent.h"
        // #include "alt_classes/CPlayerRequestControlEvent.h"
        // #include "alt_classes/CPlayerDimensionChangeEvent.h"
        // #include "alt_classes/CPlayerChangeInteriorEvent.h"
        // #include "alt_classes/CConnectionQueueAddEvent.h"
        // #include "alt_classes/CConnectionQueueRemoveEvent.h"

        // #include "alt_classes/CVehicleAttachEvent.h"
        // #include "alt_classes/CVehicleDetachEvent.h"
        // #include "alt_classes/CVehicleDestroyEvent.h"
        // #include "alt_classes/CVehicleDamageEvent.h"
        // #include "alt_classes/CVehicleHornEvent.h"
        // #include "alt_classes/CVehicleSirenEvent.h"

        // #include "alt_classes/CStartProjectileEvent.h"
        // #include "alt_classes/CFireEvent.h"
        // #include "alt_classes/CExplosionEvent.h"

        // #include "alt_classes/IConnectionInfo.h"
        // #include "alt_classes/VehicleModelInfo.h"
        // #include "alt_classes/PedModelInfo.h"

        // #include "alt_classes/CMetaChangeEvent.h"
        // #include "alt_classes/CGlobalMetaDataChangeEvent.h"
        // #include "alt_classes/CGlobalSyncedMetaDataChangeEvent.h"
        // #include "alt_classes/CSyncedMetaDataChangeEvent.h"
        // #include "alt_classes/CStreamSyncedMetaDataChangeEvent.h"
        // #include "alt_classes/CLocalMetaDataChangeEvent.h"

        // #include "alt_classes/CResourceStopEvent.h"
        // #include "alt_classes/CResourceStartEvent.h"

        name!(alt_bridge)

        generate_ns!("ICore")
        generate_ns!("IMarker")

        // generate_ns!("CNetOwnerChangeEvent")

        // generate_ns!("CPlayerDeathEvent")
        // generate_ns!("CPlayerDamageEvent")
        // generate_ns!("CPlayerEnteringVehicleEvent")
        // generate_ns!("CPlayerEnterVehicleEvent")
        // generate_ns!("CPlayerLeaveVehicleEvent")
        // generate_ns!("CPlayerChangeAnimationEvent")
        // generate_ns!("CPlayerChangeVehicleSeatEvent")
        // generate_ns!("CPlayerWeaponChangeEvent")
        // generate_ns!("CPlayerConnectDeniedEvent")
        // generate_ns!("CPlayerSpawnEvent")
        // generate_ns!("CPlayerRequestControlEvent")
        // generate_ns!("CPlayerDimensionChangeEvent")
        // generate_ns!("CPlayerChangeInteriorEvent")
        // generate_ns!("CConnectionQueueAddEvent")
        // generate_ns!("CConnectionQueueRemoveEvent")

        // generate_ns!("CVehicleAttachEvent")
        // generate_ns!("CVehicleDetachEvent")
        // generate_ns!("CVehicleDestroyEvent")
        // generate_ns!("CVehicleDamageEvent")
        // generate_ns!("CVehicleHornEvent")
        // generate_ns!("CVehicleSirenEvent")

        // generate_ns!("CStartProjectileEvent")
        // generate_ns!("CExplosionEvent")
        // generate_ns!("CFireEvent")

        // generate_ns!("IConnectionInfo")

        // generate_ns!("CMetaChangeEvent")
        // generate_ns!("CGlobalMetaDataChangeEvent")
        // generate_ns!("CGlobalSyncedMetaDataChangeEvent")
        // generate_ns!("CSyncedMetaDataChangeEvent")
        // generate_ns!("CStreamSyncedMetaDataChangeEvent")
        // generate_ns!("CLocalMetaDataChangeEvent")

        // generate_ns!("CResourceStopEvent")
        // generate_ns!("CResourceStartEvent")

        // defined in alt_bridge
        generate_ns!("events")

        generate!("set_alt_core")
        generate!("get_alt_core")
        generate!("create_script_runtime")
        generate!("register_script_runtime")

        // mvalue
        generate!("ConstMValueWrapper")
        generate!("copy_const_mvalue")
        generate!("copy_mut_mvalue")
        generate!("copy_mvalue_dict_pair")
        generate!("convert_mvalue_mut_wrapper_to_const")
        generate!("create_mvalue_vec")
        generate!("read_mvalue_type")
        generate!("read_mvalue_bool")
        generate!("read_mvalue_double")
        generate!("read_mvalue_string")
        generate!("read_mvalue_int")
        generate!("read_mvalue_uint")
        generate!("read_mvalue_list")
        generate!("read_mvalue_dict")
        generate!("read_mvalue_dict_pair_key")
        generate!("read_mvalue_dict_pair_value")
        generate!("read_mvalue_base_object")
        generate!("read_mvalue_vector3")
        generate!("read_mvalue_vector2")
        generate!("read_mvalue_byte_array_size")
        generate!("read_mvalue_byte_array")
        generate!("read_mvalue_rgba")

        generate!("create_mvalue_bool")
        generate!("create_mvalue_double")
        generate!("create_mvalue_string")
        generate!("create_mvalue_nil")
        generate!("create_mvalue_int")
        generate!("create_mvalue_uint")
        generate!("create_mvalue_list")
        generate!("push_to_mvalue_list")
        generate!("create_mvalue_dict")
        generate!("push_to_mvalue_dict")
        generate!("create_mvalue_base_object")
        generate!("create_mvalue_vector3")
        generate!("create_mvalue_vector2")
        generate!("create_mvalue_byte_array")
        generate!("create_mvalue_rgba")

        // events
        // generate!("trigger_local_event")
        // generate!("trigger_client_event")
        // generate!("trigger_client_event_for_some")
        // generate!("trigger_client_event_for_all")
        // generate!("trigger_client_event_unreliable")
        // generate!("trigger_client_event_unreliable_for_some")
        // generate!("trigger_client_event_unreliable_for_all")

        generate_ns!("base_object")
        generate_ns!("world_object")
        generate_ns!("entity")
        generate_ns!("player")
        generate_ns!("vehicle")
        generate_ns!("ped")
        generate_ns!("network_object")
        generate_ns!("col_shape")
        generate_ns!("virtual_entity")
        generate_ns!("virtual_entity_group")
        generate_ns!("blip")
        generate_ns!("voice_channel")
        generate_ns!("marker")
        generate_ns!("checkpoint")
        generate_ns!("connection_info")

        // alt::Prop
        generate!("read_alt_prop")
        // alt::DlcProp
        generate!("read_alt_dlc_prop")

        // alt::Cloth
        generate!("read_alt_cloth")
        // alt::DlcCloth
        generate!("read_alt_dlc_cloth")

        // alt::HeadOverlay
        generate!("read_alt_head_overlay")

        // alt::HeadBlendData
        generate!("read_alt_head_blend_data")

        // Vector3Wrapper
        generate!("Vector3Wrapper")
        generate!("read_vector3")

        // Vector2Wrapper
        generate!("Vector2Wrapper")
        generate!("read_vector2")

        // RGBAWrapper
        generate!("RGBAWrapper")
        generate!("read_rgba")

        generate!("WeaponWrapper")
        generate!("read_weapon")
        generate!("read_weapon_components")

        generate!("FireInfoWrapper")
        generate!("read_fire_info_pos")
        generate!("read_fire_info_weapon_hash")

        // alt::VehicleModelInfo
        generate_ns!("VehicleModelInfo")
        generate!("is_vehicle_model_info_valid")
        generate!("read_vehicle_model_info")
        generate!("read_vehicle_model_info_title")
        generate!("read_vehicle_model_info_bones")

        // alt::BoneInfo
        generate!("read_bone_info")
        generate!("read_bone_info_name")

        // alt::PedModelInfo
        generate_ns!("PedModelInfo")
        generate!("is_ped_model_info_valid")
        generate!("read_ped_model_info_bones")
        generate!("read_ped_model_info_hash")
        generate!("read_ped_model_info_name")
        generate!("read_ped_model_info_type")
        generate!("read_ped_model_info_dlc_name")
        generate!("read_ped_model_info_movement_clip_set")
        generate!("read_ped_model_info_default_unarmed_weapon")

        // alt::WeaponModelInfo
        generate_ns!("WeaponModelInfo")
        generate!("is_weapon_model_info_valid")
        generate!("read_weapon_model_info")
        generate!("read_weapon_model_info_name")
        generate!("read_weapon_model_info_ammo_type")
        generate!("read_weapon_model_info_model_name")
        generate!("read_weapon_model_info_ammo_model_name")

        // alt::Quaternion
        generate!("read_quaternion")

        generate!("read_base_object_ptr_wrapper")
        generate!("create_base_object_vec")
        generate!("push_to_base_object_vec")

        generate!("read_player_ptr_wrapper")
        generate!("create_player_vec")
        generate!("push_to_player_vec")

        generate!("read_resource_ptr_wrapper")

        generate!("create_vector2_vec")
        generate!("push_to_vector2_vec")

        generate!("ConfigDictPairWrapper")
        generate_ns!("config_node")

        generate!("create_mvalue_unordered_map")
        generate!("push_to_mvalue_unordered_map")

        generate!("read_streamed_entity_key")
        generate!("read_streamed_entity_value")
    }
    pub use alt_bridge::*;

    #[repr(transparent)]
    pub struct ResourceStartCallback(pub extern "C" fn(name: &str, full_main_path: &str));
    impl_extern_type_callback!(ResourceStartCallback, "callbacks::ResourceStartCallback");

    #[repr(transparent)]
    pub struct ResourceStopCallback(pub extern "C" fn(name: &str));
    impl_extern_type_callback!(ResourceStopCallback, "callbacks::ResourceStopCallback");

    #[repr(transparent)]
    pub struct RuntimeResourceDestroyImplCallback(pub extern "C" fn());
    impl_extern_type_callback!(
        RuntimeResourceDestroyImplCallback,
        "callbacks::RuntimeResourceDestroyImplCallback"
    );

    #[repr(transparent)]
    pub struct RuntimeOnTickCallback(pub extern "C" fn());
    impl_extern_type_callback!(RuntimeOnTickCallback, "callbacks::RuntimeOnTickCallback");

    #[repr(transparent)]
    pub struct ResourceOnEventCallback(
        pub extern "C" fn(name: &str, event: *const alt_bridge::alt::CEvent),
    );
    impl_extern_type_callback!(
        ResourceOnEventCallback,
        "callbacks::ResourceOnEventCallback"
    );

    #[repr(transparent)]
    pub struct ResourceOnCreateBaseObjectCallback(
        pub extern "C" fn(name: &str, base_object: *mut alt_bridge::alt::IBaseObject),
    );
    impl_extern_type_callback!(
        ResourceOnCreateBaseObjectCallback,
        "callbacks::ResourceOnCreateBaseObjectCallback"
    );

    #[repr(transparent)]
    pub struct ResourceOnRemoveBaseObjectCallback(
        pub extern "C" fn(name: &str, base_object: *mut alt_bridge::alt::IBaseObject),
    );
    impl_extern_type_callback!(
        ResourceOnRemoveBaseObjectCallback,
        "callbacks::ResourceOnRemoveBaseObjectCallback"
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

pub type BaseObjectMutPtr = NonNull<ffi::alt::IBaseObject>;
pub type BaseObjectRawMutPtr = *mut ffi::alt::IBaseObject;

pub mod helpers;

// auto generated files:

macro_rules! include_out_dir {
    ($file_name:literal) => {
        include!(concat!(env!("OUT_DIR"), $file_name));
    };
}

include_out_dir!("/cpp_sdk_version.rs");
include_out_dir!("/base_object_type.rs");
include_out_dir!("/event_type.rs");
include_out_dir!("/mvalue_type.rs");
include_out_dir!("/col_shape_type.rs");
include_out_dir!("/blip_type.rs");
include_out_dir!("/marker_type.rs");
include_out_dir!("/player_body_part.rs");
include_out_dir!("/player_connect_denied_reason.rs");
include_out_dir!("/explosion_type.rs");
include_out_dir!("/vehicle_model_type.rs");
include_out_dir!("/config_value_type.rs");
