use resource_impl::events::{Event, PublicEventType, SDKEventType};

pub use resource_impl::events::{
    PlayerConnectController, PlayerDisconnectController, ServerStartedController,
};

macro_rules! on_event {
    ($func_name: ident, $event_path: path, $controller: ty, $public_type: path, $sdk_type: path) => {
        pub fn $func_name(handler: impl FnMut($controller) + 'static + Sync + Send) {
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
