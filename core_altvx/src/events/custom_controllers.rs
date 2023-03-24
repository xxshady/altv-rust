use crate::resource::Resource;

use super::sdk_controllers::ColshapeEvent;

#[derive(Debug)]
pub struct VehicleEnterColShape {}

impl VehicleEnterColShape {
    pub fn new(controller: &ColshapeEvent, _: &Resource) -> Self {
        Self {}
    }
}
