use core_altvx::events::*;

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

// macro_rules! on_event {
//     ($func_name: ident, $event_path: path, $controller: ty, $sdk_type: path) => {
//         pub fn $func_name(handler: impl FnMut($controller) + 'static) {
//             core_altvx::events::add_handler($sdk_type, $event_path(Box::new(handler)));
//         }
//     };
// }

// macro_rules! on_extra_event {
//     ($func_name: ident, $event_path: path, $controller: ty, $public_event: expr, $extra_type: path, $sdk_type: path) => {
//         pub fn $func_name(handler: impl FnMut($controller) + 'static) {
//             core_altvx::events::add_extra_handler(
//                 $public_event,
//                 $extra_type,
//                 $sdk_type,
//                 $event_path(Box::new(handler)),
//             );
//         }
//     };
// }

// on_event!(
//     on_server_started,
//     Event::ServerStarted,
//     ServerStartedController,
//     SDKEventType::SERVER_STARTED
// );

// on_event!(
//     on_player_connect,
//     Event::PlayerConnect,
//     PlayerConnectController,
//     SDKEventType::PLAYER_CONNECT
// );

// on_event!(
//     on_player_disconnect,
//     Event::PlayerDisconnect,
//     PlayerDisconnectController,
//     SDKEventType::PLAYER_DISCONNECT
// );

// on_event!(
//     on_console_command,
//     Event::ConsoleCommand,
//     ConsoleCommandController,
//     SDKEventType::CONSOLE_COMMAND_EVENT
// );

// on_extra_event!(
//     on_vehicle_enter_col_shape,
//     ExtraEvent::VehicleEnterColShape,
//     VehicleEnterColShapeController,
//     Event::ColShape,
//     ExtraEventType::VehicleEnterColShape,
//     SDKEventType::COLSHAPE_EVENT
// );
