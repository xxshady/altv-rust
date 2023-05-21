pub use core_shared::*;

pub use crate::{
    alt_resource::AltResource as Resource,
    base_objects::{
        blip::{Blip, BlipContainer},
        checkpoint::{Checkpoint, CheckpointContainer},
        col_shape::{ColShape, ColShapeContainer},
        extra_pools::{AnyEntity, Entity, SyncId},
        marker::{Marker, MarkerContainer},
        network_object::{NetworkObject, NetworkObjectContainer},
        ped::{Ped, PedContainer},
        player::{Player, PlayerContainer},
        vehicle::{Vehicle, VehicleContainer},
        virtual_entity::{VirtualEntity, VirtualEntityContainer},
        virtual_entity_group::{VirtualEntityGroup, VirtualEntityGroupContainer},
        voice_channel::{VoiceChannel, VoiceChannelContainer},
        AnyBaseObject, BaseObjectPoolFuncs, BaseObjectWrapper, ValidBaseObject,
    },
    col_shape::ColShapy,
    helpers::{hash, Hash},
    init,
    ped_model_info::PedModelInfo,
    quaternion::Quaternion,
    result::{IntoVoidResult, SomeResult, VoidResult},
    rgba::RGBA,
    structs::{
        AnimationFlags, AttachToEntityBoneIndex, AttachToEntityBoneName, PlayAnimation,
        PlayerDateTime, PlayerHeadBlendData,
    },
    timers::create_timer,
    vector::{Vector2, Vector3},
    vehicle_model_info::VehicleModelInfo,
    world_object::WorldObject,
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
    // intended for public macros
    pub mod __internal {
        pub use crate::{
            // client_events::{
            //     emit_all_clients, emit_all_clients_unreliable,
            //     emit_all_clients_unreliable_without_args, emit_all_clients_without_args,
            //     emit_client, emit_client_unreliable, emit_client_unreliable_without_args,
            //     emit_client_without_args, emit_some_clients, emit_some_clients_unreliable,
            //     emit_some_clients_unreliable_without_args, emit_some_clients_without_args,
            // },
            // script_events::{emit_local_event, emit_local_event_without_args},
        };
    }

    pub use crate::{
        // client_events::emit_all_clients,
        events::{
            add_custom_handler, add_sdk_handler, connection_queue::ConnectionQueueInfo,
            custom_contexts, sdk_contexts, structs::FireInfo, CustomHandler, SDKHandler,
        },
        script_events::{
            on_client, on, ClientEventContext, LocalEventContext, emit,
        },
    };
}

pub mod core_funcs {
    pub use crate::core_funcs::*;
}

pub mod config_node {
    pub use crate::config_node::*;
}

// pub mod meta {
//     pub use crate::meta::{
//         base_object::{entry::*, normal_meta::*, synced_meta::*},
//         checkpoint_stream_synced_meta::*,
//         entity_stream_synced_meta::*,
//         entry::*,
//         global::*,
//         player_local_meta::*,
//         ve_stream_synced_meta::*,
//     };
// }

pub mod base_object {
    pub use crate::base_object_funcs::*;
}

// TEST
pub use mvalue as __mvalue;
pub use serde as __serde;
