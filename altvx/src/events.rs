use resource_impl::events::*;

pub use resource_impl::events::{
    ConsoleCommandController, PlayerConnectController, PlayerDisconnectController,
    ServerStartedController,
};

pub use resource_impl::emit_local_event as emit;
pub use resource_impl::mvalue::MValue;
pub use resource_impl::mvalue::None;
pub use resource_impl::mvalue_dict as dict;
pub use resource_impl::mvalue_list as list;
pub use resource_impl::resource_impl::on;

macro_rules! on_event {
    ($func_name: ident, $event_path: path, $controller: ty, $public_type: path, $sdk_type: path) => {
        pub fn $func_name(handler: impl FnMut($controller) + 'static) {
            resource_impl::resource_impl::add_event_handler(
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
