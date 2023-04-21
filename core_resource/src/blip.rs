use crate::{
    base_objects::{blip, player},
    helpers::{self},
    rgba::RGBA,
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
        blip.set_scale(radius).unwrap();
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
        Ok(helpers::read_cpp_vector2(unsafe {
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

    pub fn set_display(&self, display: c_int) -> VoidResult {
        //todo c_int?
        unsafe { sdk::IBlip::SetDisplay(self.raw_ptr()?, display) }
        Ok(())
    }

    pub fn sprite(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IBlip::GetSprite(self.raw_ptr()?) }.into())
    }

    pub fn color(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IBlip::GetColor(self.raw_ptr()?) }.into())
    }

    pub fn secondary_color(&self) -> SomeResult<RGBA> {
        Ok(helpers::read_cpp_rgba(
            unsafe { sdk::IBlip::GetSecondaryColor(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn alpha(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IBlip::GetAlpha(self.raw_ptr()?) }.into())
    }

    pub fn flash_timer(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IBlip::GetFlashTimer(self.raw_ptr()?) }.into())
    }

    pub fn flash_interval(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IBlip::GetFlashInterval(self.raw_ptr()?) }.into())
    }

    pub fn as_friendly(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetAsFriendly(self.raw_ptr()?) })
    }

    pub fn route(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetRoute(self.raw_ptr()?) })
    }

    pub fn bright(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetBright(self.raw_ptr()?) })
    }

    pub fn number(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IBlip::GetNumber(self.raw_ptr()?) }.into())
    }

    pub fn show_cone(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetShowCone(self.raw_ptr()?) })
    }

    pub fn flashes(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetFlashes(self.raw_ptr()?) })
    }

    pub fn flashes_alternate(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetFlashesAlternate(self.raw_ptr()?) })
    }

    pub fn as_short_range(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetAsShortRange(self.raw_ptr()?) })
    }

    pub fn priority(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IBlip::GetPriority(self.raw_ptr()?) }.into())
    }

    pub fn rotation(&self) -> SomeResult<f32> {
        Ok(unsafe { sdk::IBlip::GetRotation(self.raw_ptr()?) }.into())
    }

    pub fn gxt_name(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IBlip::GetGxtName(self.raw_ptr()?) }.to_string())
    }

    pub fn name(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IBlip::GetName(self.raw_ptr()?) }.to_string())
    }

    pub fn route_color(&self) -> SomeResult<RGBA> {
        Ok(helpers::read_cpp_rgba(
            unsafe { sdk::IBlip::GetRouteColor(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn pulse(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetPulse(self.raw_ptr()?) })
    }

    pub fn as_mission_creator(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetAsMissionCreator(self.raw_ptr()?) })
    }

    pub fn tick_visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetTickVisible(self.raw_ptr()?) })
    }

    pub fn heading_indicator_visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetHeadingIndicatorVisible(self.raw_ptr()?) })
    }

    pub fn outline_indicator_visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetOutlineIndicatorVisible(self.raw_ptr()?) })
    }

    pub fn friend_indicator_visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetFriendIndicatorVisible(self.raw_ptr()?) })
    }

    pub fn crew_indicator_visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetCrewIndicatorVisible(self.raw_ptr()?) })
    }

    pub fn category(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IBlip::GetCategory(self.raw_ptr()?) }.into())
    }

    pub fn as_high_detail(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetAsHighDetail(self.raw_ptr()?) })
    }

    pub fn shrinked(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetShrinked(self.raw_ptr()?) })
    }

    pub fn destroy(&self) -> VoidResult {
        blip::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

impl WorldObject for blip::Blip {}
