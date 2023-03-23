use crate::{helpers::get_player_from_event, resource::Resource};

use super::controllers;

use altv_sdk::ffi as sdk;

macro_rules! call_user_handlers {
    ($handler_variant: path, $handlers: expr, $controller: expr) => {
        for h in $handlers {
            if let $handler_variant(h) = h {
                h($controller);
            } else {
                panic!(
                    "expected {} handler, received: {:?}",
                    stringify!($handler_variant),
                    h
                );
            }
        }
    };
}

// TODO: better name, parser? deserializer?
pub trait SDKEvent {
    fn handle(
        &self,
        event: altv_sdk::CEventPtr,
        handlers: &mut [super::SDKHandler],
        resource: &Resource,
    );
}

pub struct ServerStarted;
impl SDKEvent for ServerStarted {
    fn handle(
        &self,
        _: altv_sdk::CEventPtr,
        handlers: &mut [super::SDKHandler],
        resource: &Resource,
    ) {
        call_user_handlers!(
            super::SDKHandler::ServerStarted,
            handlers,
            &controllers::ServerStarted {}
        );
    }
}

pub struct PlayerConnect;
impl SDKEvent for PlayerConnect {
    fn handle(
        &self,
        event: altv_sdk::CEventPtr,
        handlers: &mut [super::SDKHandler],
        resource: &Resource,
    ) {
        call_user_handlers!(
            super::SDKHandler::PlayerConnect,
            handlers,
            &controllers::PlayerConnect {
                player: get_player_from_event(
                    unsafe { sdk::events::to_CPlayerConnectEvent(event) },
                    resource,
                    sdk::CPlayerConnectEvent::GetTarget,
                ),
            }
        );
    }
}

pub struct ColshapeEvent;
impl SDKEvent for ColshapeEvent {
    fn handle(
        &self,
        _: altv_sdk::CEventPtr,
        handlers: &mut [super::SDKHandler],
        resource: &Resource,
    ) {
        call_user_handlers!(
            super::SDKHandler::ColshapeEvent,
            handlers,
            &controllers::ColshapeEvent {}
        );
    }
}

pub trait CustomEvent {
    fn handle(
        &self,
        event: altv_sdk::CEventPtr,
        handlers: &mut [super::CustomHandler],
        resource: &Resource,
    );
}

pub struct VehicleEnterColShape;
impl CustomEvent for VehicleEnterColShape {
    fn handle(
        &self,
        event: altv_sdk::CEventPtr,
        handlers: &mut [super::CustomHandler],
        resource: &Resource,
    ) {
        // TODO:
        todo!();
    }
}
