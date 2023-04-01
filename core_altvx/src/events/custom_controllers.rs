use crate::resource::Resource;

use super::sdk_controllers::ColshapeEvent;

#[derive(Debug)]
pub struct VehicleEnterColShape {}

impl VehicleEnterColShape {
    pub fn new(controller: &ColshapeEvent, resource: &Resource) -> Option<Self> {
        let true = controller.state else { return None };

        Some(Self {})
    }
}

#[derive(Debug)]
pub struct VehicleLeaveColShape {}

impl VehicleLeaveColShape {
    pub fn new(controller: &ColshapeEvent, resource: &Resource) -> Option<Self> {
        let false = controller.state else { return None };

        Some(Self {})
    }
}
