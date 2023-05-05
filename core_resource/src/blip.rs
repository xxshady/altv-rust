use crate::{
    base_objects::{blip, extra_pools::AnyEntity, player},
    helpers,
    rgba::RGBA,
    sdk,
    vector::{Vector2, Vector3},
    SomeResult, VoidResult,
};

use crate::resource::Resource;
use autocxx::prelude::*;

/// # **`Blip implementation`**
impl blip::Blip {
    pub fn new_area(pos: impl Into<Vector3>, width: f32, height: f32) -> blip::BlipContainer {
        let pos = pos.into();

        let blip = helpers::create_base_object!(
            blip,
            sdk::ICore::CreateBlip(
                std::ptr::null_mut(),
                altv_sdk::BlipType::Area as u8,
                pos.x(),
                pos.y(),
                pos.z(),
            ),
            panic!("Failed to create Area Blip")
        );

        blip.set_scale((width, height)).unwrap();
        blip
    }

    pub fn new_radius(pos: impl Into<Vector3>, radius: f32) -> blip::BlipContainer {
        let pos = pos.into();

        let blip = helpers::create_base_object!(
            blip,
            sdk::ICore::CreateBlip(
                std::ptr::null_mut(),
                altv_sdk::BlipType::Radius as u8,
                pos.x(),
                pos.y(),
                pos.z(),
            ),
            panic!("Failed to create Radius Blip")
        );

        blip.set_scale(radius).unwrap();
        blip
    }

    pub fn new_point(pos: impl Into<Vector3>) -> blip::BlipContainer {
        let pos = pos.into();

        helpers::create_base_object!(
            blip,
            sdk::ICore::CreateBlip(
                std::ptr::null_mut(),
                altv_sdk::BlipType::Destination as u8,
                pos.x(),
                pos.y(),
                pos.z(),
            ),
            panic!("Failed to create Point Blip")
        )
    }

    pub fn new_entity_point(entity: impl Into<AnyEntity>) -> SomeResult<blip::BlipContainer> {
        let entity = entity.into().raw_ptr()?;

        Ok(helpers::create_base_object!(
            blip,
            sdk::ICore::CreateBlip1(
                std::ptr::null_mut(),
                altv_sdk::BlipType::Destination as u8,
                entity,
            ),
            panic!("Failed to create Attached Blip")
        ))
    }

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

    pub fn attached_to(&self) -> SomeResult<Option<AnyEntity>> {
        let raw_ptr = self.raw_ptr()?;

        Ok(Resource::with(|resource| {
            helpers::get_entity_by_ptr(unsafe { sdk::IBlip::AttachedTo(raw_ptr) }, resource)
        }))
    }

    pub fn attach_to(&self, entity: impl Into<AnyEntity>) -> VoidResult {
        let entity = entity.into();

        unsafe { sdk::IBlip::AttachTo(self.raw_ptr()?, entity.raw_ptr()?) };
        Ok(())
    }

    pub fn blip_type(&self) -> SomeResult<altv_sdk::BlipType> {
        let raw = unsafe { sdk::IBlip::GetBlipType(self.raw_ptr()?) };
        Ok(altv_sdk::BlipType::try_from(raw).unwrap())
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

    pub fn set_display(&self, display: i32) -> VoidResult {
        unsafe { sdk::IBlip::SetDisplay(self.raw_ptr()?, display.into()) }
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

    pub fn rot(&self) -> SomeResult<f32> {
        Ok(unsafe { sdk::IBlip::GetRotation(self.raw_ptr()?) })
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

    pub fn set_sprite(&self, sprite: i32) -> VoidResult {
        unsafe { sdk::IBlip::SetSprite(self.raw_ptr()?, sprite.into()) }
        Ok(())
    }

    pub fn set_color(&self, color: i32) -> VoidResult {
        unsafe { sdk::IBlip::SetColor(self.raw_ptr()?, color.into()) }
        Ok(())
    }

    pub fn set_route(&self, state: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetRoute(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn set_route_color(&self, color: impl Into<RGBA>) -> VoidResult {
        let color = color.into();

        unsafe {
            sdk::IBlip::SetRouteColor(self.raw_ptr()?, color.r(), color.g(), color.b(), color.a())
        }
        Ok(())
    }

    pub fn set_secondary_color(&self, color: impl Into<RGBA>) -> VoidResult {
        let color = color.into();

        unsafe {
            sdk::IBlip::SetSecondaryColor(
                self.raw_ptr()?,
                color.r(),
                color.g(),
                color.b(),
                color.a(),
            )
        }
        Ok(())
    }

    pub fn set_alpha(&self, alpha: i32) -> VoidResult {
        unsafe { sdk::IBlip::SetAlpha(self.raw_ptr()?, alpha.into()) }
        Ok(())
    }

    pub fn set_flash_timer(&self, timer: i32) -> VoidResult {
        unsafe { sdk::IBlip::SetFlashTimer(self.raw_ptr()?, timer.into()) }
        Ok(())
    }

    pub fn set_flash_interval(&self, interval: i32) -> VoidResult {
        unsafe { sdk::IBlip::SetFlashInterval(self.raw_ptr()?, interval.into()) }
        Ok(())
    }

    pub fn set_as_friendly(&self, friendly: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetAsFriendly(self.raw_ptr()?, friendly) }
        Ok(())
    }

    pub fn set_bright(&self, bright: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetBright(self.raw_ptr()?, bright) }
        Ok(())
    }

    pub fn set_number(&self, number: i32) -> VoidResult {
        unsafe { sdk::IBlip::SetNumber(self.raw_ptr()?, number.into()) }
        Ok(())
    }

    pub fn set_show_cone(&self, state: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetShowCone(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn set_flashes(&self, state: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetFlashes(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn set_flashes_alternate(&self, state: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetFlashesAlternate(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn set_as_short_range(&self, state: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetAsShortRange(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn set_priority(&self, state: i32) -> VoidResult {
        unsafe { sdk::IBlip::SetPriority(self.raw_ptr()?, state.into()) }
        Ok(())
    }

    pub fn set_rot(&self, rot: f32) -> VoidResult {
        unsafe { sdk::IBlip::SetRotation(self.raw_ptr()?, rot) }
        Ok(())
    }

    pub fn set_gxt_name(&self, name: String) -> VoidResult {
        unsafe { sdk::IBlip::SetGxtName(self.raw_ptr()?, name) }
        Ok(())
    }

    pub fn set_name(&self, name: String) -> VoidResult {
        unsafe { sdk::IBlip::SetName(self.raw_ptr()?, name) }
        Ok(())
    }

    pub fn set_pulse(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetPulse(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn set_as_mission_creator(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetAsMissionCreator(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn set_tick_visible(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetTickVisible(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn set_heading_indicator_visible(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetHeadingIndicatorVisible(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn set_outline_indicator_visible(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetOutlineIndicatorVisible(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn set_friend_indicator_visible(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetFriendIndicatorVisible(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn set_crew_indicator_visible(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetCrewIndicatorVisible(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn set_category(&self, val: i32) -> VoidResult {
        unsafe { sdk::IBlip::SetCategory(self.raw_ptr()?, val.into()) }
        Ok(())
    }

    pub fn set_as_high_detail(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetAsHighDetail(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn set_shrinked(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetShrinked(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn fade(&self, opacity: u32, duration: u32) -> VoidResult {
        unsafe { sdk::IBlip::Fade(self.raw_ptr()?, opacity, duration) }
        Ok(())
    }

    pub fn destroy(&self) -> VoidResult {
        blip::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}
