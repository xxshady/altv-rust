use altv_sdk::ffi as sdk;

use crate::{helpers::get_player_from_event, player::PlayerContainer, resource::Resource};

#[derive(Debug)]
pub struct PlayerConnect {
    pub player: PlayerContainer,
}

impl PlayerConnect {
    pub fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        Self {
            player: get_player_from_event(
                unsafe { sdk::events::to_CPlayerConnectEvent(event) },
                resource,
                sdk::CPlayerConnectEvent::GetTarget,
            ),
        }
    }
}

#[derive(Debug)]
pub struct ServerStarted {}

impl ServerStarted {
    pub fn new(event_ptr: altv_sdk::CEventPtr, _: &Resource) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct ColshapeEvent {}

impl ColshapeEvent {
    pub fn new(event_ptr: altv_sdk::CEventPtr, _: &Resource) -> Self {
        Self {}
    }
}
