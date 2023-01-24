// pub enum Event {
//     PlayerConnect(fn(player: crate::player::Player)),
//     PlayerDisconnect(fn(player: crate::player::Player, reason: String)),
// }

// pub fn on(event: resource_api::events::SDKEvent) {
//     crate::RESOURCE_API
//         .get()
//         .unwrap()
//         .try_lock()
//         .unwrap()
//         .add_event_handler(event);
// }
