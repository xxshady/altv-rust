use crate::{col_shape::ColShapeContainer, resource::Resource};

use super::sdk_controllers::ColshapeEvent;

#[derive(Debug)]
pub struct VehicleEnterColShape {
    pub col_shape: ColShapeContainer,
}

impl VehicleEnterColShape {
    pub fn new(controller: &ColshapeEvent, resource: &Resource) -> Option<Self> {
        let true = controller.state else { return None };

        Some(Self { col_shape: todo!() })
    }
}

#[derive(Debug)]
pub struct VehicleLeaveColShape {
    pub col_shape: ColShapeContainer,
}

impl VehicleLeaveColShape {
    pub fn new(controller: &ColshapeEvent, resource: &Resource) -> Option<Self> {
        let false = controller.state else { return None };

        Some(Self { col_shape: todo!() })
    }
}
