// pub enum Event {
//     PlayerConnect(fn(player: crate::player::Player)),
//     PlayerDisconnect(fn(player: crate::player::Player, reason: String)),
// }

use resource_impl::resource_impl::ResourceImpl;

pub use resource_impl::events::SDKEvent as __test_SDKEvent;

pub fn on(event: resource_impl::events::SDKEvent) {
    ResourceImpl::instance().add_sdk_event_handler(event);
}
