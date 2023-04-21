use crate::{
    base_objects::{blip, player},
    helpers::{self, read_cpp_vector2},
    sdk,
    vector::{Vector2, Vector3},
    world_object::WorldObject,
    SomeResult, VoidResult,
};

use autocxx::prelude::*;
use std::ptr::NonNull;

impl blip::Blip {
    pub fn new_area(pos: impl Into<Vector3>, width: f32, height: f32) -> blip::BlipContainer {
        let pos = pos.into();

        let ptr = unsafe {
            sdk::ICore::CreateBlip(
                std::ptr::null_mut(),
                altv_sdk::BlipType::Area as u8,
                pos.x(),
                pos.y(),
                pos.z(),
            )
        };

        let blip = blip::add_to_pool!(NonNull::new(ptr).unwrap());
        blip.set_scale((width, height)).unwrap();
        blip
    }

    pub fn new_radius(pos: impl Into<Vector3>, radius: f32) -> blip::BlipContainer {
        let pos = pos.into();

        let ptr = unsafe {
            sdk::ICore::CreateBlip(
                std::ptr::null_mut(),
                altv_sdk::BlipType::Radius as u8,
                pos.x(),
                pos.y(),
                pos.z(),
            )
        };

        let blip = blip::add_to_pool!(NonNull::new(ptr).unwrap());
        blip.set_scale((radius, radius)).unwrap();
        blip
    }

    pub fn new_point(pos: impl Into<Vector3>) -> blip::BlipContainer {
        let pos = pos.into();

        let ptr = unsafe {
            sdk::ICore::CreateBlip(
                std::ptr::null_mut(),
                altv_sdk::BlipType::Destination as u8,
                pos.x(),
                pos.y(),
                pos.z(),
            )
        };
        blip::add_to_pool!(NonNull::new(ptr).unwrap())
    }

    //todo
    // pub fn new_point1(entity)

    pub fn id(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IBlip::GetID(self.raw_ptr()?) })
    }

    pub fn is_global(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::IsGlobal(self.raw_ptr()?) })
    }

    pub fn target(&self) -> SomeResult<Option<player::PlayerContainer>> {
        helpers::get_any_option_base_object!(sdk::IBlip::GetTarget(self.raw_ptr()?), player)
    }

    pub fn attached(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::IsAttached(self.raw_ptr()?) })
    }

    //todo AttachedTo

    //todo AttachTo

    // TODO: cache blip type somehow
    pub fn blip_type(&self) -> SomeResult<altv_sdk::BlipType> {
        let raw = unsafe { sdk::IBlip::GetBlipType(self.raw_ptr()?) };
        Ok(altv_sdk::BlipType::from(raw).unwrap())
    }

    pub fn scale(&self) -> SomeResult<Vector2> {
        Ok(read_cpp_vector2(unsafe {
            sdk::IBlip::GetScaleXY(self.raw_ptr()?).within_unique_ptr()
        }))
    }

    pub fn set_scale(&self, scale: impl Into<Vector2>) -> VoidResult {
        let scale = scale.into();

        unsafe { sdk::IBlip::SetScaleXY(self.raw_ptr()?, scale.x(), scale.y()) }
        Ok(())
    }

    pub fn display(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IBlip::GetDisplay(self.raw_ptr()?) }.into())
    }

    pub fn destroy(&self) -> VoidResult {
        blip::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

impl WorldObject for blip::Blip {}
