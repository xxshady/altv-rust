use crate::{
    base_objects::{
        extra_pools::{AnyWorldObject, AnyColShape},
        player, vehicle,
    },
    resource::Resource,
    helpers,
};

use super::sdk_contexts::{ColshapeEvent, ResourceStart, ResourceStop, VoiceConnectionEvent};

macro_rules! entity_enter_or_leave_col_shape {
    ($bool_state:literal, $key_name:ident, $any_entity:path) => {
        pub fn new(context: &ColshapeEvent, resource: &Resource) -> Option<Self> {
            if context.state != $bool_state {
                return None;
            };

            let $any_entity($key_name) = &context.world_object else {
                logger::debug!(
                    "{}_enter_or_leave_col_shape received {:?} -> skip",
                    stringify!($key_name),
                    &context.world_object
                );
                return None;
            };
            let $key_name = $key_name.clone();

            Some(Self {
                col_shape: helpers::get_col_shape_by_ptr(context.col_shape.as_ptr(), resource)
                    .unwrap(),
                $key_name,
            })
        }
    };
}

#[derive(Debug)]
pub struct VehicleEnterColShape {
    pub col_shape: AnyColShape,
    pub vehicle: vehicle::VehicleContainer,
}

impl VehicleEnterColShape {
    entity_enter_or_leave_col_shape!(true, vehicle, AnyWorldObject::Vehicle);
}

#[derive(Debug)]
pub struct VehicleLeaveColShape {
    pub col_shape: AnyColShape,
    pub vehicle: vehicle::VehicleContainer,
}

impl VehicleLeaveColShape {
    entity_enter_or_leave_col_shape!(false, vehicle, AnyWorldObject::Vehicle);
}

#[derive(Debug)]
pub struct PlayerEnterColShape {
    pub col_shape: AnyColShape,
    pub player: player::PlayerContainer,
}

impl PlayerEnterColShape {
    entity_enter_or_leave_col_shape!(true, player, AnyWorldObject::Player);
}

#[derive(Debug)]
pub struct PlayerLeaveColShape {
    pub col_shape: AnyColShape,
    pub player: player::PlayerContainer,
}

impl PlayerLeaveColShape {
    entity_enter_or_leave_col_shape!(false, player, AnyWorldObject::Player);
}

#[derive(Debug)]
pub struct ThisResourceStart {}

impl ThisResourceStart {
    pub fn new(context: &ResourceStart, _: &Resource) -> Option<Self> {
        Resource::with(|v| {
            if v.name != context.resource.name {
                return None;
            }
            Some(Self {})
        })
    }
}

#[derive(Debug)]
pub struct ThisResourceStop {}

impl ThisResourceStop {
    pub fn new(context: &ResourceStop, _: &Resource) -> Option<Self> {
        Resource::with(|v| {
            if v.name != context.resource.name {
                return None;
            }
            Some(Self {})
        })
    }
}

#[derive(Debug)]
pub struct VoiceConnect {}

impl VoiceConnect {
    pub fn new(context: &VoiceConnectionEvent, _: &Resource) -> Option<Self> {
        if context.state == altv_sdk::VoiceConnectionState::Connected {
            return Some(Self {});
        }
        None
    }
}

#[derive(Debug)]
pub struct VoiceDisconnect {}

impl VoiceDisconnect {
    pub fn new(context: &VoiceConnectionEvent, _: &Resource) -> Option<Self> {
        if context.state == altv_sdk::VoiceConnectionState::Disconnected {
            return Some(Self {});
        }
        None
    }
}

#[derive(Debug)]
pub struct VoiceConnecting {}

impl VoiceConnecting {
    pub fn new(context: &VoiceConnectionEvent, _: &Resource) -> Option<Self> {
        if context.state == altv_sdk::VoiceConnectionState::Connecting {
            return Some(Self {});
        }
        None
    }
}
