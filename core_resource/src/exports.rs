pub use anyhow;
pub use core_shared::*;

pub use crate::{
    alt_resource::AltResource as Resource,
    base_objects::{
        blip::{Blip, BlipContainer},
        col_shape::{ColShape, ColShapeContainer},
        extra_pools::{wrappers::AnyEntity, Entity, SyncId},
        marker::{Marker, MarkerContainer},
        meta::{BaseObjectMeta, LocalMeta, StreamSyncedMeta, SyncedMeta},
        player::{Player, PlayerContainer},
        vehicle::{Vehicle, VehicleContainer},
        virtual_entity::{VirtualEntity, VirtualEntityContainer},
        virtual_entity_group::{VirtualEntityGroup, VirtualEntityGroupContainer},
        voice_channel::{VoiceChannel, VoiceChannelContainer},
        ValidBaseObject,
    },
    helpers::{hash, Hash},
    init,
    ped_model_info::PedModelInfo,
    quaternion::Quaternion,
    rgba::RGBA,
    structs::{
        AnimationFlags, AttachToEntityBoneIndex, AttachToEntityBoneName, PlayAnimation,
        PlayerDateTime, PlayerHeadBlendData,
    },
    timers::create_timer,
    vector::{Vector2, Vector3},
    vehicle_model_info::VehicleModelInfo,
    world_object::WorldObject,
    SomeResult, VoidResult,
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
            client_events::{
                emit_all_clients, emit_all_clients_unreliable,
                emit_all_clients_unreliable_without_args, emit_all_clients_without_args,
                emit_client, emit_client_unreliable, emit_client_unreliable_without_args,
                emit_client_without_args, emit_some_clients, emit_some_clients_unreliable,
                emit_some_clients_unreliable_without_args, emit_some_clients_without_args,
            },
            script_events::{emit_local_event, emit_local_event_without_args},
        };
    }

    pub use crate::{
        client_events::emit_all_clients,
        events::{
            add_custom_handler, add_sdk_handler, connection_queue::ConnectionQueueInfo,
            custom_controllers, sdk_controllers, structs::FireInfo, CustomHandler, SDKHandler,
        },
        script_events::{add_client_handler, add_local_handler},
    };

    #[macro_export]
    macro_rules! __emit_client {
        ($event_name: expr, $player: expr) => {
            $crate::exports::events::__internal::emit_client_without_args($event_name, $player)
        };
        ($event_name: expr, $player: expr, $( $arg: expr ),+ ) => {
            (|| -> $crate::exports::VoidResult {
                let vec = $crate::exports::mvalue::mvalue_list!($( $arg ),+)?;
                $crate::exports::events::__internal::emit_client(
                    $event_name,
                    $player,
                    vec
                )
            })()
        };
    }
    pub use __emit_client as emit_client;

    #[macro_export]
    macro_rules! __emit_client_unreliable {
        ($event_name: expr, $player: expr) => {
            $crate::exports::events::__internal::emit_client_unreliable_without_args($event_name, $player)
        };
        ($event_name: expr, $player: expr, $( $arg: expr ),+ ) => {
            (|| -> $crate::exports::VoidResult {
                let vec = $crate::exports::mvalue::mvalue_list!($( $arg ),+)?;
                $crate::exports::events::__internal::emit_client_unreliable(
                    $event_name,
                    $player,
                    vec
                )
            })()
        };
    }
    pub use __emit_client_unreliable as emit_client_unreliable;

    #[macro_export]
    macro_rules! __emit_all_clients {
        ($event_name: expr) => {
            $crate::exports::events::__internal::emit_all_clients($event_name)
        };
        ($event_name: expr, $( $arg: expr ),+ ) => {
            (|| -> $crate::VoidResult {
                let vec = $crate::exports::mvalue::mvalue_list!($( $arg ),+)?;
                $crate::exports::events::__internal::emit_all_clients(
                    $event_name,
                    vec
                );
                Ok(())
            })()
        };
    }
    pub use __emit_all_clients as emit_all_clients;

    #[macro_export]
    macro_rules! __emit_all_clients_unreliable {
        ($event_name: expr) => {
            $crate::exports::events::__internal::emit_all_clients_unreliable($event_name)
        };
        ($event_name: expr, $( $arg: expr ),+ ) => {
            (|| -> $crate::VoidResult {
                let vec = $crate::exports::mvalue::mvalue_list!($( $arg ),+)?;
                $crate::exports::events::__internal::emit_all_clients_unreliable(
                    $event_name,
                    vec
                );
                Ok(())
            })()
        };
    }
    pub use __emit_all_clients_unreliable as emit_all_clients_unreliable;

    #[macro_export]
    macro_rules! __emit_some_clients {
        ($event_name: expr, $players: expr) => {
            $crate::exports::events::__internal::emit_some_clients_without_args($event_name, $players)
        };
        ($event_name: expr, $players: expr, $( $arg: expr ),+ ) => {
            (|| -> $crate::VoidResult {
                let vec = $crate::exports::mvalue::mvalue_list!($( $arg ),+)?;
                $crate::exports::events::__internal::emit_some_clients(
                    $event_name,
                    $players,
                    vec
                )
            })()
        };
    }
    pub use __emit_some_clients as emit_some_clients;

    #[macro_export]
    macro_rules! __emit_some_clients_unreliable {
        ($event_name: expr, $players: expr) => {
            $crate::exports::events::__internal::emit_some_clients_unreliable_without_args($event_name, $players)
        };
        ($event_name: expr, $players: expr, $( $arg: expr ),+ ) => {
            (|| -> $crate::VoidResult {
                let vec = $crate::exports::mvalue::mvalue_list!($( $arg ),+)?;
                $crate::exports::events::__internal::emit_some_clients_unreliable(
                    $event_name,
                    $players,
                    vec
                )
            })()
        };
    }
    pub use __emit_some_clients_unreliable as emit_some_clients_unreliable;

    /// Examples
    ///
    /// ```rust
    /// altv::events::emit!("example");
    /// ```
    ///
    /// Sending primitives
    /// ```rust
    /// altv::events::emit!("example", 1, 2, 3).unwrap();
    /// ```
    ///
    /// Sending lists
    /// ```rust
    /// altv::events::emit!("example", altv::events::list![1, 2, 3]).unwrap();
    /// ```
    #[macro_export]
    macro_rules! __emit {
        ($event_name: expr) => {
            unsafe { $crate::exports::events::__internal::emit_local_event_without_args($event_name) };
        };
        ($event_name: expr, $($arg: expr),+ $(,)*) => {
            (|| -> $crate::exports::VoidResult {
                let vec = $crate::exports::mvalue::mvalue_list!($( $arg ),+)?;
                $crate::exports::events::__internal::emit_local_event(
                    $event_name,
                    vec
                );
                Ok(())
            })()
        };
    }
    pub use __emit as emit;
}

pub mod mvalue {
    // intended for public macros
    pub mod __internal {
        pub use crate::mvalue::{serialize_mvalue, Serializable};
    }

    pub use crate::mvalue::{MValue, MValueList, MValueNone};

    #[macro_export]
    macro_rules! __mvalue_list {
        ($($arg: expr),+ $(,)*) => {
            (|| {
                let mut vec = vec![];
                $(
                    $crate::exports::mvalue::__internal::serialize_mvalue!($arg, vec);
                )+
                Ok(vec)
            })()
        };
    }
    pub use __mvalue_list as mvalue_list;

    #[macro_export]
    macro_rules! __mvalue_dict {
        ($($key: expr => $value: expr),+ $(,)*) => {
            (||{
                let mut hash_map = std::collections::HashMap::new();
                $(
                    let serializable = $crate::exports::mvalue::__internal::Serializable::try_from($value);
                    match serializable {
                        Ok(serialized) => {
                            hash_map.insert($key.to_string(), serialized);
                        }
                        Err(error) => {
                            $crate::exports::anyhow::bail!(
                                "Failed to convert value: {} to mvalue, error: {}",
                                stringify!($value),
                                error
                            );
                        }
                    }
                )+
                Ok(hash_map)
            })()
        };
    }
    pub use __mvalue_dict as mvalue_dict;
}

pub mod core_funcs {
    pub use crate::core_funcs::*;
}

pub mod config_node {
    pub use crate::config_node::*;
}
