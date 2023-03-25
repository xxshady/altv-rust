use crate::{
    base_object_maps::{self, BaseObjectMap},
    col_shape::ColShapeContainer,
    resource::Resource,
    vehicle::VehicleContainer,
};
use altv_sdk::ffi as sdk;

use super::sdk_controllers::ColshapeEvent;

#[derive(Debug)]
pub struct VehicleEnterColShape {
    pub col_shape: ColShapeContainer,
    pub vehicle: VehicleContainer,
}

impl VehicleEnterColShape {
    pub fn new(controller: &ColshapeEvent, resource: &Resource) -> Option<Self> {
        let true = controller.state else { return None };

        Some(Self {
            col_shape: base_object_maps::get_col_shape!(
                unsafe { sdk::col_shape::to_base_object(controller.col_shape) },
                resource
            )
            .unwrap(),
            vehicle: base_object_maps::get_vehicle!(
                unsafe { sdk::entity::to_base_object(controller.entity) },
                resource
            )
            .unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct VehicleLeaveColShape {
    pub col_shape: ColShapeContainer,
    pub vehicle: VehicleContainer,
}

impl VehicleLeaveColShape {
    pub fn new(controller: &ColshapeEvent, resource: &Resource) -> Option<Self> {
        let false = controller.state else { return None };

        Some(Self {
            col_shape: base_object_maps::get_col_shape!(
                unsafe { sdk::col_shape::to_base_object(controller.col_shape) },
                resource
            )
            .unwrap(),
            vehicle: base_object_maps::get_vehicle!(
                unsafe { sdk::entity::to_base_object(controller.entity) },
                resource
            )
            .unwrap(),
        })
    }
}
