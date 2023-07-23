use crate::{
    base_objects::{blip, extra_pools::AnyEntity, player},
    helpers,
    rgba::Rgba,
    sdk,
    vector::{Vector2, Vector3},
    SomeResult, VoidResult,
};

use crate::resource::Resource;
use anyhow::bail;
use autocxx::prelude::*;

/// # **`Blip implementation`**
impl blip::Blip {
    pub fn new_global_area(
        pos: impl Into<Vector3>,
        width: f32,
        height: f32,
    ) -> SomeResult<blip::BlipContainer> {
        Self::new_area_internal(true, pos, width, height, &[])
    }

    pub fn new_area_with_targets(
        pos: impl Into<Vector3>,
        width: f32,
        height: f32,
        targets: &[player::PlayerContainer],
    ) -> SomeResult<blip::BlipContainer> {
        Self::new_area_internal(false, pos, width, height, targets)
    }

    fn new_area_internal(
        global: bool,
        pos: impl Into<Vector3>,
        width: f32,
        height: f32,
        targets: &[player::PlayerContainer],
    ) -> SomeResult<blip::BlipContainer> {
        let blip = Self::new(global, altv_sdk::BlipType::Area, pos, targets)?;

        blip.set_scale((width, height)).unwrap();
        Ok(blip)
    }

    pub fn new_global_radius(
        pos: impl Into<Vector3>,
        radius: f32,
    ) -> SomeResult<blip::BlipContainer> {
        Self::new_radius_internal(true, pos, radius, &[])
    }

    pub fn new_radius_with_targets(
        pos: impl Into<Vector3>,
        radius: f32,
        targets: &[player::PlayerContainer],
    ) -> SomeResult<blip::BlipContainer> {
        Self::new_radius_internal(false, pos, radius, targets)
    }

    fn new_radius_internal(
        global: bool,
        pos: impl Into<Vector3>,
        radius: f32,
        targets: &[player::PlayerContainer],
    ) -> SomeResult<blip::BlipContainer> {
        let blip = Self::new(global, altv_sdk::BlipType::Radius, pos, targets)?;

        blip.set_scale(radius).unwrap();
        Ok(blip)
    }

    pub fn new_global_point(pos: impl Into<Vector3>) -> SomeResult<blip::BlipContainer> {
        Self::new(true, altv_sdk::BlipType::Destination, pos, &[])
    }

    pub fn new_point_with_targets(
        pos: impl Into<Vector3>,
        targets: &[player::PlayerContainer],
    ) -> SomeResult<blip::BlipContainer> {
        Self::new(false, altv_sdk::BlipType::Destination, pos, targets)
    }

    pub fn new_global_entity_point(
        entity: impl Into<AnyEntity>,
    ) -> SomeResult<blip::BlipContainer> {
        Self::new_entity_point_internal(true, entity, &[])
    }

    pub fn new_entity_point_with_targets(
        entity: impl Into<AnyEntity>,
        targets: &[player::PlayerContainer],
    ) -> SomeResult<blip::BlipContainer> {
        Self::new_entity_point_internal(false, entity, targets)
    }

    fn new_entity_point_internal(
        global: bool,
        entity: impl Into<AnyEntity>,
        targets: &[player::PlayerContainer],
    ) -> SomeResult<blip::BlipContainer> {
        let entity = entity.into().raw_ptr()?;
        let targets = helpers::convert_player_slice_to_cpp_vec(targets)?;

        Ok(helpers::create_base_object!(
            blip,
            sdk::ICore::CreateBlip1(
                global,
                altv_sdk::BlipType::Destination as u8,
                entity,
                targets
            ),
            panic!("Failed to create Entity Point Blip")
        ))
    }

    fn new(
        global: bool,
        blip_type: altv_sdk::BlipType,
        pos: impl Into<Vector3>,
        targets: &[player::PlayerContainer],
    ) -> SomeResult<blip::BlipContainer> {
        let pos = pos.into();
        let targets = helpers::convert_player_slice_to_cpp_vec(targets)?;

        Ok(helpers::create_base_object!(
            blip,
            sdk::ICore::CreateBlip(global, blip_type as u8, pos.x(), pos.y(), pos.z(), targets,),
            bail!("Failed to create {blip_type:?} Blip")
        ))
    }

    pub fn destroy(&self) -> VoidResult {
        blip::remove_from_pool!(self)?;
        self.internal_destroy()
    }

    pub fn global(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::IsGlobal(self.raw_ptr()?) })
    }

    pub fn set_global(&self, state: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetGlobal(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn targets(&self) -> SomeResult<Vec<player::PlayerContainer>> {
        let raw_ptr = self.raw_ptr()?;

        Ok(Resource::with(|resource| {
            unsafe { sdk::IBlip::GetTargets(raw_ptr) }
                .into_iter()
                .map(|v| {
                    helpers::get_non_null_player(
                        unsafe { sdk::read_player_ptr_wrapper(v) },
                        resource,
                    )
                })
                .collect()
        }))
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

    pub fn set_blip_type(&self, blip_type: altv_sdk::BlipType) -> VoidResult {
        unsafe { sdk::IBlip::SetBlipType(self.raw_ptr()?, blip_type as u8) }
        Ok(())
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

    pub fn display(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IBlip::GetDisplay(self.raw_ptr()?) })
    }

    pub fn set_display(&self, display: u32) -> VoidResult {
        unsafe { sdk::IBlip::SetDisplay(self.raw_ptr()?, display) }
        Ok(())
    }

    pub fn secondary_color(&self) -> SomeResult<Rgba> {
        Ok(helpers::read_cpp_rgba(
            unsafe { sdk::IBlip::GetSecondaryColor(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn set_secondary_color(&self, color: impl Into<Rgba>) -> VoidResult {
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

    pub fn alpha(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IBlip::GetAlpha(self.raw_ptr()?) })
    }

    pub fn set_alpha(&self, alpha: u32) -> VoidResult {
        unsafe { sdk::IBlip::SetAlpha(self.raw_ptr()?, alpha) }
        Ok(())
    }

    pub fn flash_timer(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IBlip::GetFlashTimer(self.raw_ptr()?) }.into())
    }

    pub fn set_flash_timer(&self, timer: i32) -> VoidResult {
        unsafe { sdk::IBlip::SetFlashTimer(self.raw_ptr()?, timer.into()) }
        Ok(())
    }

    pub fn flash_interval(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IBlip::GetFlashInterval(self.raw_ptr()?) }.into())
    }

    pub fn set_flash_interval(&self, interval: i32) -> VoidResult {
        unsafe { sdk::IBlip::SetFlashInterval(self.raw_ptr()?, interval.into()) }
        Ok(())
    }

    pub fn as_friendly(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetAsFriendly(self.raw_ptr()?) })
    }

    pub fn set_as_friendly(&self, friendly: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetAsFriendly(self.raw_ptr()?, friendly) }
        Ok(())
    }

    pub fn route(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetRoute(self.raw_ptr()?) })
    }

    pub fn set_route(&self, state: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetRoute(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn bright(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetBright(self.raw_ptr()?) })
    }

    pub fn set_bright(&self, bright: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetBright(self.raw_ptr()?, bright) }
        Ok(())
    }

    pub fn number(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IBlip::GetNumber(self.raw_ptr()?) }.into())
    }

    pub fn set_number(&self, number: i32) -> VoidResult {
        unsafe { sdk::IBlip::SetNumber(self.raw_ptr()?, number.into()) }
        Ok(())
    }

    pub fn show_cone(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetShowCone(self.raw_ptr()?) })
    }

    pub fn set_show_cone(&self, state: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetShowCone(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn flashes(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetFlashes(self.raw_ptr()?) })
    }

    pub fn set_flashes(&self, state: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetFlashes(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn flashes_alternate(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetFlashesAlternate(self.raw_ptr()?) })
    }

    pub fn set_flashes_alternate(&self, state: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetFlashesAlternate(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn as_short_range(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetAsShortRange(self.raw_ptr()?) })
    }

    pub fn set_as_short_range(&self, state: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetAsShortRange(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn priority(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IBlip::GetPriority(self.raw_ptr()?) })
    }

    pub fn set_priority(&self, state: u32) -> VoidResult {
        unsafe { sdk::IBlip::SetPriority(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn rot(&self) -> SomeResult<f32> {
        Ok(unsafe { sdk::IBlip::GetRotation(self.raw_ptr()?) })
    }

    pub fn set_rot(&self, rot: f32) -> VoidResult {
        unsafe { sdk::IBlip::SetRotation(self.raw_ptr()?, rot) }
        Ok(())
    }

    pub fn gxt_name(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IBlip::GetGxtName(self.raw_ptr()?) }.to_string())
    }

    pub fn set_gxt_name(&self, name: String) -> VoidResult {
        unsafe { sdk::IBlip::SetGxtName(self.raw_ptr()?, name) }
        Ok(())
    }

    pub fn name(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IBlip::GetName(self.raw_ptr()?) }.to_string())
    }

    pub fn set_name(&self, name: String) -> VoidResult {
        unsafe { sdk::IBlip::SetName(self.raw_ptr()?, name) }
        Ok(())
    }

    pub fn route_color(&self) -> SomeResult<Rgba> {
        Ok(helpers::read_cpp_rgba(
            unsafe { sdk::IBlip::GetRouteColor(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn set_route_color(&self, color: impl Into<Rgba>) -> VoidResult {
        let color = color.into();

        unsafe {
            sdk::IBlip::SetRouteColor(self.raw_ptr()?, color.r(), color.g(), color.b(), color.a())
        }
        Ok(())
    }

    pub fn pulse(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetPulse(self.raw_ptr()?) })
    }

    pub fn set_pulse(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetPulse(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn as_mission_creator(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetAsMissionCreator(self.raw_ptr()?) })
    }

    pub fn set_as_mission_creator(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetAsMissionCreator(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn tick_visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetTickVisible(self.raw_ptr()?) })
    }

    pub fn set_tick_visible(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetTickVisible(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn heading_indicator_visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetHeadingIndicatorVisible(self.raw_ptr()?) })
    }

    pub fn set_heading_indicator_visible(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetHeadingIndicatorVisible(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn outline_indicator_visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetOutlineIndicatorVisible(self.raw_ptr()?) })
    }

    pub fn set_outline_indicator_visible(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetOutlineIndicatorVisible(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn friend_indicator_visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetFriendIndicatorVisible(self.raw_ptr()?) })
    }

    pub fn set_friend_indicator_visible(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetFriendIndicatorVisible(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn crew_indicator_visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetCrewIndicatorVisible(self.raw_ptr()?) })
    }

    pub fn set_crew_indicator_visible(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetCrewIndicatorVisible(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn category(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IBlip::GetCategory(self.raw_ptr()?) })
    }

    pub fn set_category(&self, val: u32) -> VoidResult {
        unsafe { sdk::IBlip::SetCategory(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn as_high_detail(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetAsHighDetail(self.raw_ptr()?) })
    }

    pub fn set_as_high_detail(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetAsHighDetail(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn shrinked(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::GetShrinked(self.raw_ptr()?) })
    }

    pub fn set_shrinked(&self, val: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetShrinked(self.raw_ptr()?, val) }
        Ok(())
    }

    pub fn sprite(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IBlip::GetSprite(self.raw_ptr()?) })
    }

    pub fn set_sprite(&self, sprite: u32) -> VoidResult {
        unsafe { sdk::IBlip::SetSprite(self.raw_ptr()?, sprite) }
        Ok(())
    }

    pub fn color(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IBlip::GetColor(self.raw_ptr()?) })
    }

    pub fn set_color(&self, color: u32) -> VoidResult {
        unsafe { sdk::IBlip::SetColor(self.raw_ptr()?, color) }
        Ok(())
    }

    pub fn fade(&self, opacity: u32, duration: u32) -> VoidResult {
        unsafe { sdk::IBlip::Fade(self.raw_ptr()?, opacity, duration) }
        Ok(())
    }

    pub fn visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::IsVisible(self.raw_ptr()?) })
    }

    pub fn set_visible(&self, state: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetVisible(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn add_target_player(&self, player: player::PlayerContainer) -> VoidResult {
        let player_raw_ptr = player.raw_ptr()?;
        unsafe { sdk::IBlip::AddTargetPlayer(self.raw_ptr()?, player_raw_ptr) }
        Ok(())
    }

    pub fn remove_target_player(&self, player: player::PlayerContainer) -> VoidResult {
        let player_raw_ptr = player.raw_ptr()?;
        unsafe { sdk::IBlip::RemoveTargetPlayer(self.raw_ptr()?, player_raw_ptr) }
        Ok(())
    }

    pub fn hidden_on_legend(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::IsVisible(self.raw_ptr()?) })
    }

    pub fn set_hidden_on_legend(&self, hidden: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetHiddenOnLegend(self.raw_ptr()?, hidden) }
        Ok(())
    }

    pub fn minimal_on_edge(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::IsMinimalOnEdge(self.raw_ptr()?) })
    }

    pub fn set_minimal_on_edge(&self, hidden: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetMinimalOnEdge(self.raw_ptr()?, hidden) }
        Ok(())
    }

    pub fn use_height_indicator_on_edge(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::IsUseHeightIndicatorOnEdge(self.raw_ptr()?) })
    }

    pub fn set_use_height_indicator_on_edge(&self, hidden: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetUseHeightIndicatorOnEdge(self.raw_ptr()?, hidden) }
        Ok(())
    }

    pub fn short_height_threshold(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IBlip::IsShortHeightThreshold(self.raw_ptr()?) })
    }

    pub fn set_short_height_threshold(&self, hidden: bool) -> VoidResult {
        unsafe { sdk::IBlip::SetShortHeightThreshold(self.raw_ptr()?, hidden) }
        Ok(())
    }
}
