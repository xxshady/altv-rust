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
    pub fn new(_: altv_sdk::CEventPtr, _: &Resource) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct ColshapeEvent {
    pub col_shape: *mut sdk::alt::IColShape,
    pub entity: *mut sdk::alt::IEntity,
    pub state: bool,
}

impl ColshapeEvent {
    pub fn new(event: altv_sdk::CEventPtr, _: &Resource) -> Self {
        let event = unsafe { sdk::events::to_CColShapeEvent(event) };

        let state = unsafe { sdk::CColShapeEvent::GetState(event) };

        let col_shape = unsafe { sdk::CColShapeEvent::GetTarget(event) };
        if col_shape.is_null() {
            panic!("sdk::CColShapeEvent::GetTarget returned null");
        }

        let entity = unsafe { sdk::CColShapeEvent::GetEntity(event) };
        if entity.is_null() {
            panic!("sdk::CColShapeEvent::GetEntity returned null");
        }

        Self {
            col_shape,
            entity,
            state,
        }
    }
}
