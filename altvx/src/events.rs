use core_altvx::events::*;

pub use core_altvx::events::{
    ConsoleCommandController, PlayerConnectController, PlayerDisconnectController,
    ServerStartedController,
};

pub use core_altvx::emit_local_event as emit;
pub use core_altvx::mvalue::MValue;
pub use core_altvx::mvalue::MValueNone;
pub use core_altvx::mvalue_dict as dict;
pub use core_altvx::mvalue_list as list;
pub use core_altvx::script_events::add_client_handler as on_client;
pub use core_altvx::script_events::add_local_handler as on;

pub use core_altvx::emit_all_clients;
pub use core_altvx::emit_client;
pub use core_altvx::emit_some_clients;

macro_rules! on_event {
    ($func_name: ident, $event_path: path, $controller: ty, $public_type: path, $sdk_type: path) => {
        pub fn $func_name(handler: impl FnMut($controller) + 'static) {
            core_altvx::events::add_handler(
                $public_type,
                $sdk_type,
                $event_path(Box::new(handler)),
            );
        }
    };
}

on_event!(
    on_server_started,
    Event::ServerStarted,
    ServerStartedController,
    PublicEventType::ServerStarted,
    SDKEventType::SERVER_STARTED
);

on_event!(
    on_player_connect,
    Event::PlayerConnect,
    PlayerConnectController,
    PublicEventType::PlayerConnect,
    SDKEventType::PLAYER_CONNECT
);

on_event!(
    on_player_disconnect,
    Event::PlayerDisconnect,
    PlayerDisconnectController,
    PublicEventType::PlayerDisconnect,
    SDKEventType::PLAYER_DISCONNECT
);

on_event!(
    on_console_command,
    Event::ConsoleCommand,
    ConsoleCommandController,
    PublicEventType::ConsoleCommand,
    SDKEventType::CONSOLE_COMMAND_EVENT
);
