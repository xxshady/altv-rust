#![allow(clippy::missing_safety_doc)]

use std::ptr::NonNull;

pub mod ffi {
    use autocxx::prelude::*;
    use cxx::{type_id, ExternType};
    use extern_type_callback_derive::ExternTypeCallback;

    include_cpp! {
        #include "alt_bridge.h"
        #include "alt_classes/ICore.h"
        #include "alt_classes/IBaseObject.h"
        #include "alt_classes/IWorldObject.h"
        #include "alt_classes/IEntity.h"
        #include "alt_classes/IPlayer.h"
        #include "alt_classes/IVehicle.h"
        #include "alt_classes/IColShape.h"
        #include "alt_classes/IBlip.h"
        #include "alt_classes/ICheckpoint.h"
        #include "alt_classes/CEvent.h"
        #include "alt_classes/CConsoleCommandEvent.h"
        #include "alt_classes/CServerScriptEvent.h"
        #include "alt_classes/CClientScriptEvent.h"
        #include "alt_classes/CPlayerConnectEvent.h"
        #include "alt_classes/CPlayerDisconnectEvent.h"
        #include "alt_classes/CColShapeEvent.h"
        #include "alt_classes/IVirtualEntity.h"
        #include "alt_classes/IVirtualEntityGroup.h"
        #include "alt_classes/CWeaponDamageEvent.h"

        // entity
        #include "alt_classes/CNetOwnerChangeEvent.h"

        // player
        #include "alt_classes/CPlayerDeathEvent.h"
        #include "alt_classes/CPlayerDamageEvent.h"
        #include "alt_classes/CPlayerEnteringVehicleEvent.h"
        #include "alt_classes/CPlayerEnterVehicleEvent.h"
        #include "alt_classes/CPlayerLeaveVehicleEvent.h"
        #include "alt_classes/CPlayerChangeAnimationEvent.h"
        #include "alt_classes/CPlayerChangeVehicleSeatEvent.h"
        #include "alt_classes/CPlayerWeaponChangeEvent.h"
        #include "alt_classes/CPlayerConnectDeniedEvent.h"
        #include "alt_classes/CPlayerSpawnEvent.h"
        #include "alt_classes/CPlayerRequestControlEvent.h"
        #include "alt_classes/CPlayerDimensionChangeEvent.h"
        #include "alt_classes/CPlayerChangeInteriorEvent.h"
        #include "alt_classes/CConnectionQueueAddEvent.h"
        #include "alt_classes/CConnectionQueueRemoveEvent.h"

        #include "alt_classes/CVehicleAttachEvent.h"
        #include "alt_classes/CVehicleDetachEvent.h"
        #include "alt_classes/CVehicleDestroyEvent.h"
        #include "alt_classes/CVehicleDamageEvent.h"
        #include "alt_classes/CVehicleHornEvent.h"
        #include "alt_classes/CVehicleSirenEvent.h"

        #include "alt_classes/CStartProjectileEvent.h"
        #include "alt_classes/CFireEvent.h"
        #include "alt_classes/CExplosionEvent.h"

        #include "alt_classes/IConnectionInfo.h"


        name!(alt_bridge)

        generate_ns!("ICore")
        generate_ns!("IBaseObject")
        generate_ns!("IWorldObject")
        generate_ns!("IEntity")
        generate_ns!("IPlayer")
        generate_ns!("IVehicle")
        generate_ns!("IColShape")
        generate_ns!("IBlip")
        generate_ns!("ICheckpoint")
        generate_ns!("CEvent")
        generate_ns!("CConsoleCommandEvent")
        generate_ns!("CServerScriptEvent")
        generate_ns!("CClientScriptEvent")
        generate_ns!("CPlayerConnectEvent")
        generate_ns!("CPlayerDisconnectEvent")
        generate_ns!("CWeaponDamageEvent")
        generate_ns!("CColShapeEvent")
        generate_ns!("IVirtualEntity")
        generate_ns!("IVirtualEntityGroup")

        generate_ns!("CNetOwnerChangeEvent")

        generate_ns!("CPlayerDeathEvent")
        generate_ns!("CPlayerDamageEvent")
        generate_ns!("CPlayerEnteringVehicleEvent")
        generate_ns!("CPlayerEnterVehicleEvent")
        generate_ns!("CPlayerLeaveVehicleEvent")
        generate_ns!("CPlayerChangeAnimationEvent")
        generate_ns!("CPlayerChangeVehicleSeatEvent")
        generate_ns!("CPlayerWeaponChangeEvent")
        generate_ns!("CPlayerConnectDeniedEvent")
        generate_ns!("CPlayerSpawnEvent")
        generate_ns!("CPlayerRequestControlEvent")
        generate_ns!("CPlayerDimensionChangeEvent")
        generate_ns!("CPlayerChangeInteriorEvent")
        generate_ns!("CConnectionQueueAddEvent")
        generate_ns!("CConnectionQueueRemoveEvent")

        generate_ns!("CVehicleAttachEvent")
        generate_ns!("CVehicleDetachEvent")
        generate_ns!("CVehicleDestroyEvent")
        generate_ns!("CVehicleDamageEvent")
        generate_ns!("CVehicleHornEvent")
        generate_ns!("CVehicleSirenEvent")

        generate_ns!("CStartProjectileEvent")
        generate_ns!("CExplosionEvent")
        generate_ns!("CFireEvent")

        generate_ns!("IConnectionInfo")

        // defined in alt_bridge
        generate_ns!("events")

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
        generate!("get_mvalue_vector3")
        generate!("get_mvalue_vector2")

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
        generate!("create_mvalue_vector3")
        generate!("create_mvalue_vector2")

        // events
        generate!("trigger_local_event")
        generate!("trigger_client_event")
        generate!("trigger_client_event_for_some")
        generate!("trigger_client_event_for_all")
        generate!("trigger_local_event")

        generate_ns!("base_object")
        generate_ns!("world_object")
        generate_ns!("entity")
        generate_ns!("player")
        generate_ns!("vehicle")
        generate_ns!("col_shape")
        generate_ns!("virtual_entity")
        generate_ns!("virtual_entity_group")

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

        generate!("create_player_vec")
        generate!("push_to_player_vec")

        generate!("create_vector2_vec")
        generate!("push_to_vector2_vec")
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

pub type BaseObjectMutPtr = NonNull<ffi::alt::IBaseObject>;
pub type BaseObjectRawMutPtr = *mut ffi::alt::IBaseObject;

pub mod helpers;

// auto generated files:

mod cpp_sdk_version;
pub use cpp_sdk_version::ALT_SDK_VERSION;

mod base_object_type;
pub use base_object_type::BaseObjectType;

mod event_type;
pub use event_type::EventType;

mod mvalue_type;
pub use mvalue_type::MValueType;

mod col_shape_type;
pub use col_shape_type::ColShapeType;

mod player_body_part;
pub use player_body_part::PlayerBodyPart;

mod player_connect_denied_reason;
pub use player_connect_denied_reason::PlayerConnectDeniedReason;

mod explosion_type;
pub use explosion_type::ExplosionType;
