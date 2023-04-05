use core_altvx::events;

use crate::anyhow;

pub use core_altvx::events::custom_controllers;
pub use core_altvx::events::sdk_controllers;

pub use core_altvx::emit_local_event as emit;
pub use core_altvx::mvalue::MValue;
pub use core_altvx::mvalue::MValueNone;
pub use core_altvx::mvalue_dict as dict;
pub use core_altvx::mvalue_list as list;
pub use core_altvx::script_events::add_client_handler as on_client;
pub use core_altvx::script_events::add_local_handler as on;

pub use core_altvx::__emit_all_clients as emit_all_clients;
pub use core_altvx::__emit_client as emit_client;
pub use core_altvx::__emit_some_clients as emit_some_clients;

macro_rules! on_sdk_event {
    ($func_name: ident, $event_name: ident) => {
        pub fn $func_name(
            handler: impl FnMut(&events::sdk_controllers::$event_name) -> anyhow::Result<()> + 'static,
        ) {
            events::add_handler(events::SDKHandler::$event_name(Box::new(handler)));
        }
    };
}

macro_rules! on_custom_event {
    ($func_name: ident, $event_name: ident) => {
        pub fn $func_name(
            handler: impl FnMut(&events::custom_controllers::$event_name) -> anyhow::Result<()>
                + 'static,
        ) {
            events::add_custom_handler(events::CustomHandler::$event_name(Box::new(handler)));
        }
    };
}

on_sdk_event!(on_server_started, ServerStarted);
on_sdk_event!(on_player_connect, PlayerConnect);
on_sdk_event!(on_player_disconnect, PlayerDisconnect);
on_sdk_event!(on_console_command, ConsoleCommandEvent);

on_custom_event!(on_vehicle_enter_col_shape, VehicleEnterColShape);
on_custom_event!(on_vehicle_leave_col_shape, VehicleLeaveColShape);
on_custom_event!(on_player_enter_col_shape, PlayerEnterColShape);
on_custom_event!(on_player_leave_col_shape, PlayerLeaveColShape);
