use crate::{
    base_objects::{col_shape, player, vehicle},
    exports::AnyEntity,
    resource::Resource,
};

use super::sdk_controllers::{ColshapeEvent, ResourceStart, ResourceStop};

macro_rules! entity_enter_or_leave_col_shape {
    ($bool_state: literal, $key_name: ident, $any_entity: path) => {
        pub fn new(controller: &ColshapeEvent, _: &Resource) -> Option<Self> {
            if controller.state != $bool_state {
                return None;
            };

            let $any_entity($key_name) = &controller.entity else {
                                        logger::debug!(
                                            "{}_enter_or_leave_col_shape received {:?} -> skip",
                                            stringify!($key_name), &controller.entity
                                        );
                                        return None;
                                    };
            let $key_name = $key_name.clone();

            Some(Self {
                col_shape: Resource::with_base_objects_mut(|v, _| {
                    v.col_shape.get_by_ptr(controller.col_shape)
                })
                .unwrap(),
                $key_name,
            })
        }
    };
}

#[derive(Debug)]
pub struct VehicleEnterColShape {
    pub col_shape: col_shape::ColShapeContainer,
    pub vehicle: vehicle::VehicleContainer,
}

impl VehicleEnterColShape {
    entity_enter_or_leave_col_shape!(true, vehicle, AnyEntity::Vehicle);
}

#[derive(Debug)]
pub struct VehicleLeaveColShape {
    pub col_shape: col_shape::ColShapeContainer,
    pub vehicle: vehicle::VehicleContainer,
}

impl VehicleLeaveColShape {
    entity_enter_or_leave_col_shape!(false, vehicle, AnyEntity::Vehicle);
}

#[derive(Debug)]
pub struct PlayerEnterColShape {
    pub col_shape: col_shape::ColShapeContainer,
    pub player: player::PlayerContainer,
}

impl PlayerEnterColShape {
    entity_enter_or_leave_col_shape!(true, player, AnyEntity::Player);
}

#[derive(Debug)]
pub struct PlayerLeaveColShape {
    pub col_shape: col_shape::ColShapeContainer,
    pub player: player::PlayerContainer,
}

impl PlayerLeaveColShape {
    entity_enter_or_leave_col_shape!(false, player, AnyEntity::Player);
}

#[derive(Debug)]
pub struct ThisResourceStart {}

impl ThisResourceStart {
    pub fn new(controller: &ResourceStart, _: &Resource) -> Option<Self> {
        Resource::with(|v| {
            if v.name != controller.resource.name {
                return None;
            }
            Some(Self {})
        })
    }
}

#[derive(Debug)]
pub struct ThisResourceStop {}

impl ThisResourceStop {
    pub fn new(controller: &ResourceStop, _: &Resource) -> Option<Self> {
        Resource::with(|v| {
            if v.name != controller.resource.name {
                return None;
            }
            Some(Self {})
        })
    }
}
