//! # alt:V events
//! Events that are emitted from alt:V for the use of the script.
//!
//! ## Examples
//! Player connect
//! ```rust
//! # fn test() -> altv::VoidResult {
//! altv::events::on_player_connect(|event| {
//!     let name = event.player.name()?;
//!     altv::log!("player with name: {name} connected!");
//!     Ok(())
//! });
//! # Ok(()) }
//! ```
//!
//! # Custom events
//! Events that are emitted from the script.
//!
//! ## Examples
//!
//! Local event (cross-resource)
//! ```rust
//! # fn test() -> altv::VoidResult {
//! # use altv::prelude::*;
//! altv::events::on("example", |event| {
//!     let args: (bool, i32) = event.args.deserialize()?;
//!     altv::log!("args: {args:?}"); // args: (true, 123)
//!     Ok(())
//! });
//!
//! altv::events::emit("example", &[&true, &123])?;
//!
//! // Event handler can also be removed later
//! let mut controller = altv::events::on("example", |event| {});
//! controller.destroy()?; // ok
//! controller.destroy()?; // err "Already destroyed"
//!
//! # Ok(()) }
//! ```
//!
//! client (player) -> server events
//! ```rust
//! # fn test() -> altv::VoidResult {
//! # use altv::prelude::*;
//! altv::events::on_player("example", |event| {
//!     let args: (bool, i32) = event.args.deserialize()?;
//!     altv::log!("event from player: {}", event.player.name()?);
//!     altv::log!("args: {args:?}"); // args: (true, 123)
//!     Ok(())
//! });
//!
//! // Event handler can also be removed later
//! let mut controller = altv::events::on_player("example", |event| {});
//! controller.destroy()?; // ok
//! controller.destroy()?; // err "Already destroyed"
//! # Ok(()) }
//! ```

use core_resource::exports::{events, IntoVoidResult};
pub use events::{
    emit, emit_all_players, emit_all_players_unreliable, emit_some_players,
    emit_some_players_unreliable, on, on_player, PlayerEventContext, FireInfo, LocalEventContext,
    ScriptEventController, LocalEventController, PlayerEventController,
};

pub use events::custom_contexts::*;
pub use events::sdk_contexts::*;

macro_rules! on_sdk_event {
    ($func_name:ident, $event_name:ident) => {
        pub fn $func_name<V: IntoVoidResult>(
            mut handler: impl FnMut(&events::sdk_contexts::$event_name) -> V + 'static,
        ) {
            events::add_sdk_handler(events::SDKHandler::$event_name(Box::new(move |c| {
                handler(c).into_void_result()
            })));
        }
    };
}

on_sdk_event!(on_server_started, ServerStarted);
on_sdk_event!(on_console_command, ConsoleCommandEvent);

on_sdk_event!(on_net_owner_change, NetownerChange);

on_sdk_event!(on_weapon_damage, WeaponDamageEvent);
on_sdk_event!(on_player_connect, PlayerConnect);
on_sdk_event!(on_player_disconnect, PlayerDisconnect);
on_sdk_event!(on_player_death, PlayerDeath);
on_sdk_event!(on_player_damage, PlayerDamage);
on_sdk_event!(on_player_entering_vehicle, PlayerEnteringVehicle);
on_sdk_event!(on_player_enter_vehicle, PlayerEnterVehicle);
on_sdk_event!(on_player_leave_vehicle, PlayerLeaveVehicle);
on_sdk_event!(on_player_change_vehicle_seat, PlayerChangeVehicleSeat);
on_sdk_event!(on_player_weapon_change, PlayerWeaponChange);
on_sdk_event!(on_player_connect_denied, PlayerConnectDenied);
on_sdk_event!(on_player_spawn, PlayerSpawn);
on_sdk_event!(on_player_request_control, PlayerRequestControl);
on_sdk_event!(on_player_dimension_change, PlayerDimensionChange);
on_sdk_event!(on_player_interior_change, PlayerChangeInteriorEvent);
on_sdk_event!(on_player_heal, PlayerHeal);

on_sdk_event!(on_vehicle_attach, VehicleAttach);
on_sdk_event!(on_vehicle_detach, VehicleDetach);
on_sdk_event!(on_vehicle_destroy, VehicleDestroy);
on_sdk_event!(on_vehicle_damage, VehicleDamage);
on_sdk_event!(on_vehicle_horn, VehicleHorn);
on_sdk_event!(on_vehicle_siren, VehicleSiren);

on_sdk_event!(on_start_projectile, StartProjectileEvent);
on_sdk_event!(on_start_fire, FireEvent);
on_sdk_event!(on_explosion, ExplosionEvent);

on_sdk_event!(on_connection_queue_add, ConnectionQueueAdd);
on_sdk_event!(on_connection_queue_remove, ConnectionQueueRemove);

on_sdk_event!(on_global_meta_change, GlobalMetaChange);
on_sdk_event!(on_global_synced_meta_change, GlobalSyncedMetaChange);
on_sdk_event!(on_synced_meta_change, SyncedMetaChange);
on_sdk_event!(on_stream_synced_meta_change, StreamSyncedMetaChange);
on_sdk_event!(on_local_meta_change, LocalSyncedMetaChange);

on_sdk_event!(on_any_resource_stop, ResourceStop);
on_sdk_event!(on_any_resource_start, ResourceStart);

on_sdk_event!(on_voice_connection, VoiceConnectionEvent);

on_sdk_event!(on_synced_scene_request, RequestSyncedScene);
on_sdk_event!(on_synced_scene_start, StartSyncedScene);
on_sdk_event!(on_synced_scene_stop, StopSyncedScene);
on_sdk_event!(on_synced_scene_update, UpdateSyncedScene);

macro_rules! on_custom_event {
    ($func_name:ident, $event_name:ident) => {
        pub fn $func_name<V: IntoVoidResult>(
            mut handler: impl FnMut(&events::custom_contexts::$event_name) -> V + 'static,
        ) {
            events::add_custom_handler(events::CustomHandler::$event_name(Box::new(move |c| {
                handler(c).into_void_result()
            })));
        }
    };
}

on_custom_event!(on_vehicle_enter_col_shape, VehicleEnterColShape);
on_custom_event!(on_vehicle_leave_col_shape, VehicleLeaveColShape);
on_custom_event!(on_player_enter_col_shape, PlayerEnterColShape);
on_custom_event!(on_player_leave_col_shape, PlayerLeaveColShape);

on_custom_event!(on_resource_start, ThisResourceStart);
on_custom_event!(on_resource_stop, ThisResourceStop);

on_custom_event!(on_voice_connect, VoiceConnect);
on_custom_event!(on_voice_disconnect, VoiceDisconnect);
on_custom_event!(on_voice_connecting, VoiceConnecting);
