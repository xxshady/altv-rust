pub use anyhow;
pub use erased_serde;
pub use mvalue;
pub use serde;
pub use serde_bytes;

pub use core_shared::*;

pub use crate::{
    alt_resource::AltResource as Resource,
    base_objects::{
        blip::{Blip, BlipContainer},
        checkpoint::{Checkpoint, CheckpointContainer},
        col_shape_circle::{ColShapeCircle, ColShapeCircleContainer},
        extra_pools::{
            AnyEntity, AnyWorldObject, Entity, SyncId, WorldObject, AnyColShape, ColShape,
        },
        marker::{Marker, MarkerContainer},
        object::{Object, ObjectContainer},
        ped::{Ped, PedContainer},
        player::{Player, PlayerContainer},
        vehicle::{Vehicle, VehicleContainer},
        virtual_entity::{VirtualEntity, VirtualEntityContainer},
        virtual_entity_group::{VirtualEntityGroup, VirtualEntityGroupContainer},
        voice_channel::{VoiceChannel, VoiceChannelContainer},
        AnyBaseObject, BaseObjectId, BaseObjectPoolFuncs, BaseObjectWrapper, ValidBaseObject,
    },
    helpers::{hash, Hash},
    init,
    ped_model_info::PedModelInfo,
    quaternion::Quaternion,
    result::{IntoVoidResult, SomeResult, VoidResult},
    rgba::Rgba,
    structs::{
        AmmoType, AnimationFlags, AttachToEntityBoneIndex, AttachToEntityBoneName, PlayAnimation,
        PlayerDateTime, PlayerHeadBlendData, AmmoFlags, Decoration,
    },
    timers::{create_timer, remove_timer, Timer},
    vector::{Vector2, Vector3},
    vehicle_model_info::VehicleModelInfo,
    weapon_model_info::WeaponModelInfo,
    mvalue_hash_map::MValueHashMap,
};

pub mod logging {
    pub use crate::logging::{log, log_error, log_warn};

    #[macro_export]
    macro_rules! __log {
        ($($arg:tt)*) => {{
            $crate::exports::logging::log(&format!($($arg)*))
        }}
    }
    pub use __log as log_macro;

    #[macro_export]
    macro_rules! __log_warn {
        ($($arg:tt)*) => {{
            $crate::exports::logging::log_warn(&format!($($arg)*))
        }}
    }
    pub use __log_warn as log_warn_macro;

    #[macro_export]
    macro_rules! __log_error {
        ($($arg:tt)*) => {{
            $crate::exports::logging::log_error(&format!($($arg)*))
        }}
    }
    pub use __log_error as log_error_macro;
}

pub mod events {
    pub use crate::{
        client_events::*,
        events::{
            add_custom_handler, add_sdk_handler, custom_contexts, sdk_contexts, structs::FireInfo,
            CustomHandler, SDKHandler,
        },
        script_events::{
            emit, on, on_player, PlayerEventContext, LocalEventContext, ScriptEventController,
            LocalEventController, PlayerEventController,
        },
    };
}

pub mod core_funcs {
    pub use crate::core_funcs::*;
}

pub mod config_node {
    pub use crate::config_node::*;
}

pub mod meta {
    pub use crate::meta::{
        base_object::{entry::*, normal_meta::*, synced_meta::*},
        checkpoint_stream_synced_meta::*,
        entity_stream_synced_meta::*,
        entry::*,
        global::*,
        player_local_meta::*,
        ve_stream_synced_meta::*,
    };
}

pub mod base_object {
    pub use crate::base_object_funcs::*;
}
