use core_altvx::{exports::events, VoidResult};
pub use events::{
    add_client_handler as on_client, add_local_handler as on, custom_controllers, emit,
    emit_all_clients, emit_client, emit_some_clients, sdk_controllers,
};

macro_rules! on_sdk_event {
    ($func_name: ident, $event_name: ident) => {
        pub fn $func_name(
            handler: impl FnMut(&events::sdk_controllers::$event_name) -> VoidResult + 'static,
        ) {
            events::add_sdk_handler(events::SDKHandler::$event_name(Box::new(handler)));
        }
    };
}

macro_rules! on_custom_event {
    ($func_name: ident, $event_name: ident) => {
        pub fn $func_name(
            handler: impl FnMut(&events::custom_controllers::$event_name) -> VoidResult + 'static,
        ) {
            events::add_custom_handler(events::CustomHandler::$event_name(Box::new(handler)));
        }
    };
}

on_sdk_event!(on_server_started, ServerStarted);
on_sdk_event!(on_console_command, ConsoleCommandEvent);

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

on_custom_event!(on_vehicle_enter_col_shape, VehicleEnterColShape);
on_custom_event!(on_vehicle_leave_col_shape, VehicleLeaveColShape);
on_custom_event!(on_player_enter_col_shape, PlayerEnterColShape);
on_custom_event!(on_player_leave_col_shape, PlayerLeaveColShape);
