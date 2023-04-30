use std::ptr::NonNull;

use autocxx::prelude::*;

use crate::{
    base_objects::{
        extra_pools::{get_entity_by_id, wrappers::AnyEntity, Entity},
        meta, player, vehicle,
    },
    helpers::{self, IntoHash, IntoString},
    quaternion::Quaternion,
    resource::Resource,
    rgba::RGBA,
    sdk, structs,
    vector::Vector3,
    SomeResult, VoidResult,
};

/// # **`Vehicle implementation`**
impl vehicle::Vehicle {
    pub fn all() -> Vec<vehicle::VehicleContainer> {
        Resource::with_base_objects_ref(|v, _| v.vehicle.all())
    }

    pub fn get_by_id(id: u32) -> SomeResult<vehicle::VehicleContainer> {
        get_entity_by_id!(AnyEntity::Vehicle, id).ok_or(anyhow::anyhow!("No vehicle with id: {id}"))
    }

    pub fn new(
        model: impl IntoHash,
        pos: impl Into<Vector3>,
        rot: impl Into<Vector3>,
    ) -> SomeResult<vehicle::VehicleContainer> {
        let pos = pos.into();
        let rot = rot.into();

        let ptr = unsafe {
            sdk::ICore::CreateVehicle(
                model.into_hash(),
                pos.x(),
                pos.y(),
                pos.z(),
                rot.x(),
                rot.y(),
                rot.z(),
            )
        };
        let Some(ptr) = NonNull::new(ptr) else {
            anyhow::bail!("Vehicle model is incorrect or there is no free id for new entity");
        };

        Ok(vehicle::add_to_pool!(ptr))
    }

    pub fn destroy(&self) -> VoidResult {
        vehicle::remove_from_pool!(self)?;
        self.internal_destroy()
    }

    pub fn driver(&self) -> SomeResult<Option<player::PlayerContainer>> {
        helpers::get_any_option_base_object!(sdk::IVehicle::GetDriver(self.raw_ptr()?), player)
    }

    pub fn attached(&self) -> SomeResult<Option<vehicle::VehicleContainer>> {
        helpers::get_any_option_base_object!(sdk::IVehicle::GetAttached(self.raw_ptr()?), vehicle)
    }

    pub fn attached_to(&self) -> SomeResult<Option<vehicle::VehicleContainer>> {
        helpers::get_any_option_base_object!(sdk::IVehicle::GetAttachedTo(self.raw_ptr()?), vehicle)
    }

    pub fn is_destroyed(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsDestroyed(self.raw_ptr()?) })
    }

    pub fn is_primary_color_rgb(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsPrimaryColorRGB(self.raw_ptr()?) })
    }

    pub fn is_secondary_color_rgb(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsSecondaryColorRGB(self.raw_ptr()?) })
    }

    pub fn is_tire_smoke_color_custom(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsTireSmokeColorCustom(self.raw_ptr()?) })
    }

    pub fn custom_tires(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::GetCustomTires(self.raw_ptr()?) })
    }

    pub fn is_extra_on(&self, extra_id: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsExtraOn(self.raw_ptr()?, extra_id) })
    }

    pub fn is_neon_active(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsNeonActive(self.raw_ptr()?) })
    }

    pub fn is_engine_on(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsEngineOn(self.raw_ptr()?) })
    }

    pub fn is_handbrake_active(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsHandbrakeActive(self.raw_ptr()?) })
    }

    pub fn is_siren_active(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsSirenActive(self.raw_ptr()?) })
    }

    pub fn is_window_opened(&self, window_id: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsWindowOpened(self.raw_ptr()?, window_id) })
    }

    pub fn is_daylight_on(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsDaylightOn(self.raw_ptr()?) })
    }

    pub fn is_nightlight_on(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsNightlightOn(self.raw_ptr()?) })
    }

    pub fn is_flamethrower_active(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsFlamethrowerActive(self.raw_ptr()?) })
    }

    pub fn is_wheel_burst(&self, wheel_id: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsWheelBurst(self.raw_ptr()?, wheel_id) })
    }

    pub fn does_wheel_has_tire(&self, wheel_id: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::DoesWheelHasTire(self.raw_ptr()?, wheel_id) })
    }

    pub fn is_wheel_detached(&self, wheel_id: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsWheelDetached(self.raw_ptr()?, wheel_id) })
    }

    pub fn is_wheel_on_fire(&self, wheel_id: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsWheelOnFire(self.raw_ptr()?, wheel_id) })
    }

    pub fn get_wheel_health(&self, wheel_id: u8) -> SomeResult<f32> {
        Ok(unsafe { sdk::IVehicle::GetWheelHealth(self.raw_ptr()?, wheel_id) })
    }

    pub fn is_light_damaged(&self, light_id: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsLightDamaged(self.raw_ptr()?, light_id) })
    }

    pub fn is_window_damaged(&self, window_id: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsWindowDamaged(self.raw_ptr()?, window_id) })
    }

    pub fn is_special_light_damaged(&self, special_light_id: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsSpecialLightDamaged(self.raw_ptr()?, special_light_id) })
    }

    pub fn has_armored_windows(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::HasArmoredWindows(self.raw_ptr()?) })
    }

    pub fn manual_engine_control(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsManualEngineControl(self.raw_ptr()?) })
    }

    pub fn set_manual_engine_control(&self, toggle: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetManualEngineControl(self.raw_ptr()?, toggle) }
        Ok(())
    }

    pub fn get_mod(&self, category: u8) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetMod(self.raw_ptr()?, category) })
    }

    pub fn get_mods_count(&self, category: u8) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetModsCount(self.raw_ptr()?, category) })
    }

    pub fn mod_kits_count(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetModKitsCount(self.raw_ptr()?) })
    }

    pub fn mod_kit(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetModKit(self.raw_ptr()?) })
    }

    pub fn set_mod_kit(&self, id: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::SetModKit(self.raw_ptr()?, id) })
    }

    pub fn primary_color(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetPrimaryColor(self.raw_ptr()?) })
    }

    pub fn set_primary_color(&self, color: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetPrimaryColor(self.raw_ptr()?, color) }
        Ok(())
    }

    pub fn secondary_color(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetSecondaryColor(self.raw_ptr()?) })
    }

    pub fn set_secondary_color(&self, color: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetSecondaryColor(self.raw_ptr()?, color) }
        Ok(())
    }

    pub fn pearl_color(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetPearlColor(self.raw_ptr()?) })
    }

    pub fn wheel_color(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetWheelColor(self.raw_ptr()?) })
    }

    pub fn set_wheel_color(&self, color: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetWheelColor(self.raw_ptr()?, color) }
        Ok(())
    }

    pub fn interior_color(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetInteriorColor(self.raw_ptr()?) })
    }

    pub fn set_interior_color(&self, color: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetInteriorColor(self.raw_ptr()?, color) }
        Ok(())
    }

    pub fn dashboard_color(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetDashboardColor(self.raw_ptr()?) })
    }

    pub fn set_dashboard_color(&self, color: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetDashboardColor(self.raw_ptr()?, color) }
        Ok(())
    }

    pub fn wheel_type(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetWheelType(self.raw_ptr()?) })
    }

    pub fn wheel_variation(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetWheelVariation(self.raw_ptr()?) })
    }

    pub fn rear_wheel_variation(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetRearWheelVariation(self.raw_ptr()?) })
    }

    pub fn set_wheels(&self, wheel_type: u8, variation: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetWheels(self.raw_ptr()?, wheel_type, variation) }
        Ok(())
    }

    pub fn set_rear_wheels(&self, variation: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetRearWheels(self.raw_ptr()?, variation) }
        Ok(())
    }

    pub fn set_custom_tires(&self, toggle: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetCustomTires(self.raw_ptr()?, toggle) }
        Ok(())
    }

    pub fn special_darkness(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetSpecialDarkness(self.raw_ptr()?) })
    }

    pub fn set_special_darkness(&self, value: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetSpecialDarkness(self.raw_ptr()?, value) }
        Ok(())
    }

    pub fn numberplate_index(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVehicle::GetNumberplateIndex(self.raw_ptr()?) })
    }

    pub fn set_numberplate_index(&self, index: u32) -> VoidResult {
        unsafe { sdk::IVehicle::SetNumberplateIndex(self.raw_ptr()?, index) }
        Ok(())
    }

    pub fn window_tint(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetWindowTint(self.raw_ptr()?) })
    }

    pub fn set_window_tint(&self, tint: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetWindowTint(self.raw_ptr()?, tint) }
        Ok(())
    }

    pub fn dirt_level(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetDirtLevel(self.raw_ptr()?) })
    }

    pub fn set_dirt_level(&self, level: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetDirtLevel(self.raw_ptr()?, level) }
        Ok(())
    }

    pub fn neon_active(&self) -> SomeResult<structs::VehicleNeon> {
        let mut left = false;
        let mut right = false;
        let mut front = false;
        let mut back = false;

        unsafe {
            sdk::IVehicle::GetNeonActive(
                self.raw_ptr()?,
                &mut left as *mut _,
                &mut right as *mut _,
                &mut front as *mut _,
                &mut back as *mut _,
            )
        }

        Ok(structs::VehicleNeon {
            left,
            right,
            front,
            back,
        })
    }

    pub fn set_neon_active(&self, left: bool, right: bool, front: bool, back: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetNeonActive(self.raw_ptr()?, left, right, front, back) }
        Ok(())
    }

    pub fn livery(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetLivery(self.raw_ptr()?) })
    }

    pub fn set_livery(&self, livery: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetLivery(self.raw_ptr()?, livery) }
        Ok(())
    }

    pub fn roof_livery(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetRoofLivery(self.raw_ptr()?) })
    }

    pub fn set_roof_livery(&self, roof_livery: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetRoofLivery(self.raw_ptr()?, roof_livery) }
        Ok(())
    }

    pub fn headlight_color(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetHeadlightColor(self.raw_ptr()?) })
    }

    pub fn radio_station_index(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVehicle::GetRadioStationIndex(self.raw_ptr()?) })
    }

    pub fn lock_state(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetLockState(self.raw_ptr()?) })
    }

    pub fn set_lock_state(&self, state: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetLockState(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn get_door_state(&self, door_id: u8) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetDoorState(self.raw_ptr()?, door_id) })
    }

    pub fn wheels_count(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetWheelsCount(self.raw_ptr()?) })
    }

    pub fn repairs_count(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetRepairsCount(self.raw_ptr()?) })
    }

    pub fn body_health(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVehicle::GetBodyHealth(self.raw_ptr()?) })
    }

    pub fn body_additional_health(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVehicle::GetBodyAdditionalHealth(self.raw_ptr()?) })
    }

    pub fn get_part_damage_level(&self, part_id: u8) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetPartDamageLevel(self.raw_ptr()?, part_id) })
    }

    pub fn get_part_bullet_holes(&self, part_id: u8) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetPartBulletHoles(self.raw_ptr()?, part_id) })
    }

    pub fn get_armored_window_shoot_count(&self, window_id: u8) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetArmoredWindowShootCount(self.raw_ptr()?, window_id) })
    }

    pub fn get_bumper_damage_level(&self, bumper_id: u8) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetBumperDamageLevel(self.raw_ptr()?, bumper_id) })
    }

    pub fn roof_closed(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::GetRoofState(self.raw_ptr()?) == 1 })
    }

    pub fn set_roof_closed(&self, toggle: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetRoofState(self.raw_ptr()?, toggle as u8) }
        Ok(())
    }

    /// At present this function is **unstable**,
    /// consider recreating the vehicle instead. <br>
    /// <https://github.com/altmp/altv-issues/issues/1748> <br>
    /// <https://github.com/altmp/altv-issues/issues/1184> <br>
    /// <https://github.com/altmp/altv-issues/issues/1445> <br>
    /// <https://github.com/altmp/altv-issues/issues/1426> <br>
    pub fn repair(&self) -> VoidResult {
        unsafe { sdk::IVehicle::SetFixed(self.raw_ptr()?) }
        Ok(())
    }

    pub fn primary_color_rgb(&self) -> SomeResult<RGBA> {
        Ok(helpers::read_cpp_rgba(
            unsafe { sdk::IVehicle::GetPrimaryColorRGB(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn set_primary_color_rgb(&self, color: impl Into<RGBA>) -> VoidResult {
        let color: RGBA = color.into();
        unsafe {
            sdk::IVehicle::SetPrimaryColorRGB(
                self.raw_ptr()?,
                color.r(),
                color.g(),
                color.b(),
                color.a(),
            )
        }
        Ok(())
    }

    pub fn secondary_color_rgb(&self) -> SomeResult<RGBA> {
        Ok(helpers::read_cpp_rgba(
            unsafe { sdk::IVehicle::GetSecondaryColorRGB(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn set_secondary_color_rgb(&self, color: impl Into<RGBA>) -> VoidResult {
        let color: RGBA = color.into();
        unsafe {
            sdk::IVehicle::SetSecondaryColorRGB(
                self.raw_ptr()?,
                color.r(),
                color.g(),
                color.b(),
                color.a(),
            )
        }
        Ok(())
    }

    pub fn tire_smoke_color(&self) -> SomeResult<RGBA> {
        Ok(helpers::read_cpp_rgba(
            unsafe { sdk::IVehicle::GetTireSmokeColor(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn set_tire_smoke_color(&self, color: impl Into<RGBA>) -> VoidResult {
        let color: RGBA = color.into();
        unsafe {
            sdk::IVehicle::SetTireSmokeColor(
                self.raw_ptr()?,
                color.r(),
                color.g(),
                color.b(),
                color.a(),
            )
        }
        Ok(())
    }

    pub fn numberplate_text(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IVehicle::GetNumberplateText(self.raw_ptr()?) }.to_string())
    }

    pub fn set_numberplate_text(&self, text: impl IntoString) -> VoidResult {
        unsafe { sdk::IVehicle::SetNumberplateText(self.raw_ptr()?, text.into_string()) }
        Ok(())
    }

    pub fn appearance_data_base64(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IVehicle::GetAppearanceDataBase64(self.raw_ptr()?) }.to_string())
    }

    pub fn set_appearance_data_base64(&self, data: impl IntoString) -> VoidResult {
        unsafe { sdk::IVehicle::LoadAppearanceDataFromBase64(self.raw_ptr()?, data.into_string()) }
        Ok(())
    }

    pub fn game_state_base64(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IVehicle::GetGameStateBase64(self.raw_ptr()?) }.to_string())
    }

    pub fn set_game_state_base64(&self, data: impl IntoString) -> VoidResult {
        unsafe { sdk::IVehicle::LoadGameStateFromBase64(self.raw_ptr()?, data.into_string()) }
        Ok(())
    }

    pub fn health_data_base64(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IVehicle::GetHealthDataBase64(self.raw_ptr()?) }.to_string())
    }

    pub fn set_health_data_base64(&self, data: impl IntoString) -> VoidResult {
        unsafe { sdk::IVehicle::LoadHealthDataFromBase64(self.raw_ptr()?, data.into_string()) }
        Ok(())
    }

    pub fn damage_data_base64(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IVehicle::GetDamageDataBase64(self.raw_ptr()?) }.to_string())
    }

    pub fn set_damage_data_base64(&self, data: impl IntoString) -> VoidResult {
        unsafe { sdk::IVehicle::LoadDamageDataFromBase64(self.raw_ptr()?, data.into_string()) }
        Ok(())
    }

    pub fn script_data_base64(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IVehicle::GetScriptDataBase64(self.raw_ptr()?) }.to_string())
    }

    pub fn set_script_data_base64(&self, data: impl IntoString) -> VoidResult {
        unsafe { sdk::IVehicle::LoadScriptDataFromBase64(self.raw_ptr()?, data.into_string()) }
        Ok(())
    }

    pub fn neon_color(&self) -> SomeResult<RGBA> {
        Ok(helpers::read_cpp_rgba(
            unsafe { sdk::IVehicle::GetNeonColor(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn set_neon_color(&self, color: impl Into<RGBA>) -> VoidResult {
        let color: RGBA = color.into();
        unsafe {
            sdk::IVehicle::SetNeonColor(self.raw_ptr()?, color.r(), color.g(), color.b(), color.a())
        }
        Ok(())
    }

    pub fn set_engine_on(&self, toggle: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetEngineOn(self.raw_ptr()?, toggle) }
        Ok(())
    }

    pub fn set_headlight_color(&self, color: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetHeadlightColor(self.raw_ptr()?, color) }
        Ok(())
    }

    pub fn set_radio_station_index(&self, index: u32) -> VoidResult {
        unsafe { sdk::IVehicle::SetRadioStationIndex(self.raw_ptr()?, index) }
        Ok(())
    }

    pub fn set_siren_active(&self, toggle: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetSirenActive(self.raw_ptr()?, toggle) }
        Ok(())
    }

    pub fn set_door_state(&self, door_id: u8, state: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetDoorState(self.raw_ptr()?, door_id, state) }
        Ok(())
    }

    pub fn set_window_opened(&self, window_id: u8, state: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetWindowOpened(self.raw_ptr()?, window_id, state) }
        Ok(())
    }

    pub fn set_lights_multiplier(&self, multiplier: f32) -> VoidResult {
        unsafe { sdk::IVehicle::SetLightsMultiplier(self.raw_ptr()?, multiplier) }
        Ok(())
    }

    pub fn set_engine_health(&self, health: i32) -> VoidResult {
        unsafe { sdk::IVehicle::SetEngineHealth(self.raw_ptr()?, health) }
        Ok(())
    }

    pub fn set_petrol_tank_health(&self, health: i32) -> VoidResult {
        unsafe { sdk::IVehicle::SetPetrolTankHealth(self.raw_ptr()?, health) }
        Ok(())
    }

    pub fn set_wheel_burst(&self, wheel_id: u8, state: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetWheelBurst(self.raw_ptr()?, wheel_id, state) }
        Ok(())
    }

    pub fn set_wheel_has_tire(&self, wheel_id: u8, state: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetWheelHasTire(self.raw_ptr()?, wheel_id, state) }
        Ok(())
    }

    pub fn set_wheel_detached(&self, wheel_id: u8, state: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetWheelDetached(self.raw_ptr()?, wheel_id, state) }
        Ok(())
    }

    pub fn set_wheel_on_fire(&self, wheel_id: u8, state: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetWheelOnFire(self.raw_ptr()?, wheel_id, state) }
        Ok(())
    }

    pub fn set_wheel_health(&self, wheel_id: u8, health: f32) -> VoidResult {
        unsafe { sdk::IVehicle::SetWheelHealth(self.raw_ptr()?, wheel_id, health) }
        Ok(())
    }

    pub fn set_wheel_fixed(&self, wheel_id: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetWheelFixed(self.raw_ptr()?, wheel_id) }
        Ok(())
    }

    pub fn set_body_health(&self, health: u32) -> VoidResult {
        unsafe { sdk::IVehicle::SetBodyHealth(self.raw_ptr()?, health) }
        Ok(())
    }

    pub fn set_body_additional_health(&self, health: u32) -> VoidResult {
        unsafe { sdk::IVehicle::SetBodyAdditionalHealth(self.raw_ptr()?, health) }
        Ok(())
    }

    pub fn set_part_damage_level(&self, part_id: u8, damage: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetPartDamageLevel(self.raw_ptr()?, part_id, damage) }
        Ok(())
    }

    pub fn set_part_bullet_holes(&self, part_id: u8, shoots_count: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetPartBulletHoles(self.raw_ptr()?, part_id, shoots_count) }
        Ok(())
    }

    pub fn set_light_damaged(&self, light_id: u8, damaged: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetLightDamaged(self.raw_ptr()?, light_id, damaged) }
        Ok(())
    }

    pub fn set_window_damaged(&self, window_id: u8, damaged: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetWindowDamaged(self.raw_ptr()?, window_id, damaged) }
        Ok(())
    }

    pub fn set_special_light_damaged(&self, special_light_id: u8, damaged: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetSpecialLightDamaged(self.raw_ptr()?, special_light_id, damaged) }
        Ok(())
    }

    pub fn set_armored_window_health(&self, window_id: u8, health: f32) -> VoidResult {
        unsafe { sdk::IVehicle::SetArmoredWindowHealth(self.raw_ptr()?, window_id, health) }
        Ok(())
    }

    pub fn set_armored_window_shoot_count(&self, window_id: u8, count: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetArmoredWindowShootCount(self.raw_ptr()?, window_id, count) }
        Ok(())
    }

    pub fn set_bumper_damage_level(&self, bumper_id: u8, damage_level: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetBumperDamageLevel(self.raw_ptr()?, bumper_id, damage_level) }
        Ok(())
    }

    pub fn drift_mode(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsDriftMode(self.raw_ptr()?) })
    }

    pub fn set_drift_mode(&self, toggle: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetDriftMode(self.raw_ptr()?, toggle) }
        Ok(())
    }

    pub fn boat_anchor_active(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsBoatAnchorActive(self.raw_ptr()?) })
    }

    pub fn set_boat_anchor_active(&self, toggle: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetBoatAnchorActive(self.raw_ptr()?, toggle) }
        Ok(())
    }

    pub fn set_search_light(
        &self,
        toggle: bool,
        spotted_entity: impl Into<AnyEntity>,
    ) -> SomeResult<bool> {
        let entity: AnyEntity = spotted_entity.into();
        Ok(unsafe { sdk::IVehicle::SetSearchLight(self.raw_ptr()?, toggle, entity.raw_ptr()?) })
    }

    pub fn light_state(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetLightState(self.raw_ptr()?) })
    }

    pub fn set_light_state(&self, state: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetLightState(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn has_timed_explosion(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::HasTimedExplosion(self.raw_ptr()?) })
    }

    pub fn timed_explosion_culprit(&self) -> SomeResult<Option<player::PlayerContainer>> {
        helpers::get_any_option_base_object!(
            sdk::IVehicle::GetTimedExplosionCulprit(self.raw_ptr()?),
            player
        )
    }

    pub fn timed_explosion_time(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVehicle::GetTimedExplosionTime(self.raw_ptr()?) })
    }

    pub fn set_timed_explosion(
        &self,
        state: bool,
        culprit: impl Into<player::PlayerContainer>,
        time: u32,
    ) -> VoidResult {
        let culprit: player::PlayerContainer = culprit.into();
        unsafe {
            sdk::IVehicle::SetTimedExplosion(self.raw_ptr()?, state, culprit.raw_ptr()?, time)
        }
        Ok(())
    }

    pub fn is_towing_disabled(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::IsTowingDisabled(self.raw_ptr()?) })
    }

    pub fn set_disable_towing(&self, disable: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetDisableTowing(self.raw_ptr()?, disable) }
        Ok(())
    }

    pub fn rocket_refuel_speed(&self) -> SomeResult<f32> {
        Ok(unsafe { sdk::IVehicle::GetRocketRefuelSpeed(self.raw_ptr()?) })
    }

    pub fn set_rocket_refuel_speed(&self, speed: f32) -> VoidResult {
        unsafe { sdk::IVehicle::SetRocketRefuelSpeed(self.raw_ptr()?, speed) }
        Ok(())
    }

    pub fn counter_measure_count(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IVehicle::GetCounterMeasureCount(self.raw_ptr()?) })
    }

    pub fn set_counter_measure_count(&self, measure_count: u32) -> VoidResult {
        unsafe { sdk::IVehicle::SetCounterMeasureCount(self.raw_ptr()?, measure_count) }
        Ok(())
    }

    pub fn script_max_speed(&self) -> SomeResult<f32> {
        Ok(unsafe { sdk::IVehicle::GetScriptMaxSpeed(self.raw_ptr()?) })
    }

    pub fn set_script_max_speed(&self, speed: f32) -> VoidResult {
        unsafe { sdk::IVehicle::SetScriptMaxSpeed(self.raw_ptr()?, speed) }
        Ok(())
    }

    pub fn weapon_capacity(&self, index: u8) -> SomeResult<i32> {
        Ok(unsafe { sdk::IVehicle::GetWeaponCapacity(self.raw_ptr()?, index) })
    }

    pub fn set_weapon_capacity(&self, index: u8, state: i32) -> VoidResult {
        unsafe { sdk::IVehicle::SetWeaponCapacity(self.raw_ptr()?, index, state) }
        Ok(())
    }

    pub fn hybrid_extra_active(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::GetHybridExtraActive(self.raw_ptr()?) })
    }

    pub fn set_hybrid_extra_active(&self, toggle: bool) -> VoidResult {
        unsafe { sdk::IVehicle::SetHybridExtraActive(self.raw_ptr()?, toggle) }
        Ok(())
    }

    pub fn hybrid_extra_state(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IVehicle::GetHybridExtraState(self.raw_ptr()?) })
    }

    pub fn set_hybrid_extra_state(&self, state: u8) -> VoidResult {
        unsafe { sdk::IVehicle::SetHybridExtraState(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn quaternion(&self) -> SomeResult<Quaternion> {
        Ok(helpers::read_cpp_quaternion(
            unsafe { sdk::IVehicle::GetQuaternion(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn set_quaternion(&self, quaternion: impl Into<Quaternion>) -> VoidResult {
        let quat = quaternion.into();
        unsafe {
            sdk::IVehicle::SetQuaternion(self.raw_ptr()?, quat.x(), quat.y(), quat.z(), quat.w())
        }
        Ok(())
    }

    pub fn velocity(&self) -> SomeResult<Vector3> {
        Ok(helpers::read_cpp_vector3(
            unsafe { sdk::IVehicle::GetVelocity(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn toggle_extra(&self, extra_id: u8, toggle: bool) -> VoidResult {
        unsafe { sdk::IVehicle::ToggleExtra(self.raw_ptr()?, extra_id, toggle) }
        Ok(())
    }

    pub fn set_mod(&self, category: u8, id: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IVehicle::SetMod(self.raw_ptr()?, category, id) })
    }
}

meta::impl_entity_meta_for!(StreamSyncedMeta, vehicle::Vehicle);
