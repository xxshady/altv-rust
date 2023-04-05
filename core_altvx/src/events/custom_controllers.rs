use crate::{
    base_objects::{col_shape, player, vehicle, AnyBaseObject},
    resource::Resource,
};

use super::sdk_controllers::ColshapeEvent;

macro_rules! entity_enter_or_leave_col_shape {
    ($bool_state: literal, $key_name: ident, $any_base_object: path) => {
        pub fn new(controller: &ColshapeEvent, resource: &Resource) -> Option<Self> {
            let $bool_state = controller.state else { return None };

            let $key_name = resource.base_objects.borrow().get_by_ptr(controller.entity);
            let Some($key_name) = $key_name else { unreachable!() };

            let $key_name = match $key_name {
                $any_base_object(v) => Some(v),
                other => {
                    logger::debug!(
                        "{}_enter_or_leave_col_shape received {other:?} -> skip",
                        stringify!($key_name)
                    );
                    None
                }
            };

            let Some($key_name) = $key_name else { return None; };

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
    entity_enter_or_leave_col_shape!(true, vehicle, AnyBaseObject::Vehicle);
}

#[derive(Debug)]
pub struct VehicleLeaveColShape {
    pub col_shape: col_shape::ColShapeContainer,
    pub vehicle: vehicle::VehicleContainer,
}

impl VehicleLeaveColShape {
    entity_enter_or_leave_col_shape!(false, vehicle, AnyBaseObject::Vehicle);
}

#[derive(Debug)]
pub struct PlayerEnterColShape {
    pub col_shape: col_shape::ColShapeContainer,
    pub player: player::PlayerContainer,
}

impl PlayerEnterColShape {
    entity_enter_or_leave_col_shape!(true, player, AnyBaseObject::Player);
}

#[derive(Debug)]
pub struct PlayerLeaveColShape {
    pub col_shape: col_shape::ColShapeContainer,
    pub player: player::PlayerContainer,
}

impl PlayerLeaveColShape {
    entity_enter_or_leave_col_shape!(false, player, AnyBaseObject::Player);
}
