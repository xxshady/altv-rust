use crate::{
    base_objects::{
        extra_pools::{AnyEntity, SyncId},
        player, vehicle,
    },
    helpers::{self, read_cpp_vector3, Hash, IntoHash},
    meta::{entity_stream_synced_meta::StreamSyncedEntityMeta, player_local_meta::LocalPlayerMeta},
    resource::Resource,
    rgba::Rgba,
    sdk, structs,
    vector::Vector3,
    SomeResult, VoidResult,
};
use autocxx::prelude::*;

pub type SquaredDistance = i32;

/// # **`Player implementation`**
impl player::Player {
    pub fn name(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IPlayer::GetName(self.raw_ptr()?) }.to_string())
    }

    pub fn social_name(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IPlayer::GetSocialClubName(self.raw_ptr()?) }.to_string())
    }

    pub fn spawn(&self, model: impl IntoHash, pos: impl Into<Vector3>) -> VoidResult {
        self.set_model(model)?;
        let pos = pos.into();
        unsafe { sdk::IPlayer::Spawn(self.raw_ptr()?, pos.x(), pos.y(), pos.z(), 0) }
        Ok(())
    }

    pub fn despawn(&self) -> VoidResult {
        unsafe { sdk::IPlayer::Despawn(self.raw_ptr()?) }
        Ok(())
    }

    pub fn clear_blood_damage(&self) -> VoidResult {
        unsafe { sdk::IPlayer::ClearBloodDamage(self.raw_ptr()?) }
        Ok(())
    }

    pub fn clear_tasks(&self) -> VoidResult {
        unsafe { sdk::IPlayer::ClearTasks(self.raw_ptr()?) }
        Ok(())
    }

    pub fn set_model(&self, model: impl IntoHash) -> VoidResult {
        unsafe { sdk::IPlayer::SetModel(self.raw_ptr()?, model.into_hash()) }
        Ok(())
    }

    pub fn set_date_time(&self, date_time: structs::PlayerDateTime) -> VoidResult {
        let structs::PlayerDateTime {
            day,
            month,
            year,
            hour,
            minute,
            second,
        } = date_time;
        unsafe {
            sdk::IPlayer::SetDateTime(
                self.raw_ptr()?,
                day.into(),
                month.into(),
                year.into(),
                hour.into(),
                minute.into(),
                second.into(),
            );
        }
        Ok(())
    }

    pub fn set_weather(&self, weather: u32) -> VoidResult {
        unsafe {
            sdk::IPlayer::SetWeather(self.raw_ptr()?, weather);
        }
        Ok(())
    }

    pub fn get_head_blend_data(&self) -> SomeResult<structs::PlayerHeadBlendData> {
        let raw = unsafe { sdk::IPlayer::GetHeadBlendData(self.raw_ptr()?) }.within_unique_ptr();

        let (
            mut shape_first_id,
            mut shape_second_id,
            mut shape_third_id,
            mut skin_first_id,
            mut skin_second_id,
            mut skin_third_id,
            mut shape_mix,
            mut skin_mix,
            mut third_mix,
        ) = Default::default();

        unsafe {
            sdk::read_alt_head_blend_data(
                raw.as_ref().unwrap(),
                &mut shape_first_id,
                &mut shape_second_id,
                &mut shape_third_id,
                &mut skin_first_id,
                &mut skin_second_id,
                &mut skin_third_id,
                &mut shape_mix,
                &mut skin_mix,
                &mut third_mix,
            )
        }

        Ok(structs::PlayerHeadBlendData {
            shape_first_id,
            shape_second_id,
            shape_third_id,
            skin_first_id,
            skin_second_id,
            skin_third_id,
            shape_mix,
            skin_mix,
            third_mix,
        })
    }

    pub fn set_head_blend_data(&self, data: structs::PlayerHeadBlendData) -> VoidResult {
        let structs::PlayerHeadBlendData {
            shape_first_id,
            shape_second_id,
            shape_third_id,
            skin_first_id,
            skin_second_id,
            skin_third_id,
            shape_mix,
            skin_mix,
            third_mix,
        } = data;

        unsafe {
            sdk::IPlayer::SetHeadBlendData(
                self.raw_ptr()?,
                shape_first_id,
                shape_second_id,
                shape_third_id,
                skin_first_id,
                skin_second_id,
                skin_third_id,
                shape_mix,
                skin_mix,
                third_mix,
            )
        }

        Ok(())
    }

    pub fn reset_head_blend_data(&self) -> VoidResult {
        self.set_head_blend_data(Default::default())
    }

    pub fn health(&self) -> SomeResult<u16> {
        Ok(unsafe { sdk::IPlayer::GetHealth(self.raw_ptr()?) })
    }

    pub fn set_health(&self, health: u16) -> VoidResult {
        unsafe { sdk::IPlayer::SetHealth(self.raw_ptr()?, health) }
        Ok(())
    }

    pub fn max_health(&self) -> SomeResult<u16> {
        Ok(unsafe { sdk::IPlayer::GetMaxHealth(self.raw_ptr()?) })
    }

    pub fn set_max_health(&self, max_health: u16) -> VoidResult {
        unsafe { sdk::IPlayer::SetMaxHealth(self.raw_ptr()?, max_health) }
        Ok(())
    }

    pub fn armour(&self) -> SomeResult<u16> {
        Ok(unsafe { sdk::IPlayer::GetArmour(self.raw_ptr()?) })
    }

    pub fn set_armour(&self, armour: u16) -> VoidResult {
        unsafe { sdk::IPlayer::SetArmour(self.raw_ptr()?, armour) }
        Ok(())
    }

    pub fn max_armour(&self) -> SomeResult<u16> {
        Ok(unsafe { sdk::IPlayer::GetMaxArmour(self.raw_ptr()?) })
    }

    pub fn set_max_armour(&self, max_armour: u16) -> VoidResult {
        unsafe { sdk::IPlayer::SetMaxArmour(self.raw_ptr()?, max_armour) }
        Ok(())
    }

    pub fn is_dead(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsDead(self.raw_ptr()?) })
    }

    pub fn is_jumping(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsJumping(self.raw_ptr()?) })
    }

    pub fn is_in_ragdoll(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsInRagdoll(self.raw_ptr()?) })
    }

    pub fn is_aiming(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsAiming(self.raw_ptr()?) })
    }

    pub fn is_shooting(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsShooting(self.raw_ptr()?) })
    }

    pub fn is_reloading(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsReloading(self.raw_ptr()?) })
    }

    pub fn is_entering_vehicle(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsEnteringVehicle(self.raw_ptr()?) })
    }

    pub fn is_leaving_vehicle(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsLeavingVehicle(self.raw_ptr()?) })
    }

    pub fn is_on_ladder(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsOnLadder(self.raw_ptr()?) })
    }

    pub fn is_in_melee(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsInMelee(self.raw_ptr()?) })
    }

    pub fn is_in_cover(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsInCover(self.raw_ptr()?) })
    }

    pub fn is_in_vehicle(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsInVehicle(self.raw_ptr()?) })
    }

    pub fn is_flashlight_active(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsFlashlightActive(self.raw_ptr()?) })
    }

    pub fn is_super_jump_enabled(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsSuperJumpEnabled(self.raw_ptr()?) })
    }

    pub fn is_crouching(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsCrouching(self.raw_ptr()?) })
    }

    pub fn is_stealthy(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsStealthy(self.raw_ptr()?) })
    }

    pub fn is_spawned(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsSpawned(self.raw_ptr()?) })
    }

    pub fn is_connected(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsConnected(self.raw_ptr()?) })
    }

    pub fn is_parachuting(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsParachuting(self.raw_ptr()?) })
    }

    pub fn has_weapon_component(
        &self,
        weapon: impl IntoHash,
        component: impl IntoHash,
    ) -> SomeResult<bool> {
        Ok(unsafe {
            sdk::IPlayer::HasWeaponComponent(
                self.raw_ptr()?,
                weapon.into_hash(),
                component.into_hash(),
            )
        })
    }

    pub fn add_weapon_component(
        &self,
        weapon: impl IntoHash,
        component: impl IntoHash,
    ) -> VoidResult {
        unsafe {
            sdk::IPlayer::AddWeaponComponent(
                self.raw_ptr()?,
                weapon.into_hash(),
                component.into_hash(),
            )
        }
        Ok(())
    }

    pub fn remove_weapon_component(
        &self,
        weapon: impl IntoHash,
        component: impl IntoHash,
    ) -> VoidResult {
        unsafe {
            sdk::IPlayer::RemoveWeaponComponent(
                self.raw_ptr()?,
                weapon.into_hash(),
                component.into_hash(),
            )
        }
        Ok(())
    }

    pub fn get_weapon_tint_index(&self, weapon: impl IntoHash) -> SomeResult<u8> {
        Ok(unsafe { sdk::IPlayer::GetWeaponTintIndex(self.raw_ptr()?, weapon.into_hash()) })
    }

    pub fn set_weapon_tint_index(&self, weapon: impl IntoHash, tint: u8) -> VoidResult {
        unsafe { sdk::IPlayer::SetWeaponTintIndex(self.raw_ptr()?, weapon.into_hash(), tint) }
        Ok(())
    }

    pub fn current_weapon(&self) -> SomeResult<Hash> {
        Ok(unsafe { sdk::IPlayer::GetCurrentWeapon(self.raw_ptr()?) })
    }

    pub fn set_current_weapon(&self, weapon: impl IntoHash) -> VoidResult {
        unsafe { sdk::IPlayer::SetCurrentWeapon(self.raw_ptr()?, weapon.into_hash()) }
        Ok(())
    }

    pub fn current_weapon_components(&self) -> SomeResult<Vec<Hash>> {
        let cpp_vector = unsafe { sdk::IPlayer::GetCurrentWeaponComponents(self.raw_ptr()?) };
        Ok(cpp_vector.into_iter().copied().collect())
    }

    pub fn current_weapon_tint_index(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IPlayer::GetCurrentWeaponTintIndex(self.raw_ptr()?) })
    }

    pub fn give_weapon(&self, weapon: impl IntoHash, ammo: i32, select_now: bool) -> VoidResult {
        unsafe { sdk::IPlayer::GiveWeapon(self.raw_ptr()?, weapon.into_hash(), ammo, select_now) }
        Ok(())
    }

    pub fn remove_weapon(&self, weapon: impl IntoHash) -> VoidResult {
        unsafe { sdk::IPlayer::RemoveWeapon(self.raw_ptr()?, weapon.into_hash()) };
        Ok(())
    }

    pub fn remove_all_weapons(&self, remove_all_ammo: bool) -> VoidResult {
        unsafe { sdk::IPlayer::RemoveAllWeapons(self.raw_ptr()?, remove_all_ammo) }
        Ok(())
    }

    pub fn weapons(&self) -> SomeResult<Vec<structs::Weapon>> {
        let raw = unsafe { sdk::IPlayer::GetWeapons(self.raw_ptr()?) };
        let weapons = raw
            .into_iter()
            .map(|v| {
                let (mut hash, mut tint_index) = Default::default();
                unsafe { sdk::read_weapon(v, &mut hash, &mut tint_index) }
                let raw_components = unsafe { sdk::read_weapon_components(v) };

                structs::Weapon {
                    hash,
                    tint_index,
                    components: raw_components.into_iter().copied().collect(),
                }
            })
            .collect();

        Ok(weapons)
    }

    pub fn move_speed(&self) -> SomeResult<f32> {
        Ok(unsafe { sdk::IPlayer::GetMoveSpeed(self.raw_ptr()?) })
    }

    pub fn aim_pos(&self) -> SomeResult<Vector3> {
        Ok(read_cpp_vector3(
            unsafe { sdk::IPlayer::GetAimPos(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn head_rot(&self) -> SomeResult<Vector3> {
        Ok(read_cpp_vector3(
            unsafe { sdk::IPlayer::GetHeadRotation(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn vehicle(&self) -> SomeResult<Option<vehicle::VehicleContainer>> {
        helpers::get_any_option_base_object!(sdk::IPlayer::GetVehicle(self.raw_ptr()?), vehicle)
    }

    pub fn seat(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IPlayer::GetSeat(self.raw_ptr()?) })
    }

    pub fn entity_aiming_at(&self) -> SomeResult<Option<AnyEntity>> {
        let ptr = unsafe { sdk::IPlayer::GetEntityAimingAt(self.raw_ptr()?) };
        Ok(Resource::with(|v| helpers::get_entity_by_ptr(ptr, v)))
    }

    pub fn entity_aim_offset(&self) -> SomeResult<Vector3> {
        Ok(read_cpp_vector3(
            unsafe { sdk::IPlayer::GetEntityAimOffset(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn current_animation_dict(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IPlayer::GetCurrentAnimationDict(self.raw_ptr()?) })
    }

    pub fn current_animation_name(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IPlayer::GetCurrentAnimationName(self.raw_ptr()?) })
    }

    pub fn forward_speed(&self) -> SomeResult<f32> {
        Ok(unsafe { sdk::IPlayer::GetForwardSpeed(self.raw_ptr()?) })
    }

    pub fn strafe_speed(&self) -> SomeResult<f32> {
        Ok(unsafe { sdk::IPlayer::GetStrafeSpeed(self.raw_ptr()?) })
    }

    pub fn ping(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IPlayer::GetPing(self.raw_ptr()?) })
    }

    pub fn ip(&self) -> SomeResult<std::net::Ipv6Addr> {
        let string_ip = unsafe { sdk::IPlayer::GetIP(self.raw_ptr()?) }.to_string();
        Ok(string_ip.parse()?)
    }

    pub fn social_id(&self) -> SomeResult<u64> {
        Ok(unsafe { sdk::IPlayer::GetSocialID(self.raw_ptr()?) })
    }

    pub fn hwid_hash(&self) -> SomeResult<u64> {
        Ok(unsafe { sdk::IPlayer::GetHwidHash(self.raw_ptr()?) })
    }

    pub fn hwid_ex_hash(&self) -> SomeResult<u64> {
        Ok(unsafe { sdk::IPlayer::GetHwidExHash(self.raw_ptr()?) })
    }

    pub fn auth_token(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IPlayer::GetAuthToken(self.raw_ptr()?) }.to_string())
    }

    pub fn discord_id(&self) -> SomeResult<i64> {
        Ok(unsafe { sdk::IPlayer::GetDiscordId(self.raw_ptr()?) })
    }

    pub fn kick(&self, reason: impl ToString) -> VoidResult {
        unsafe { sdk::IPlayer::Kick(self.raw_ptr()?, reason.to_string()) }
        Ok(())
    }

    pub fn get_clothes(&self, component: u8) -> SomeResult<structs::PlayerCloth> {
        let raw =
            unsafe { sdk::IPlayer::GetClothes(self.raw_ptr()?, component) }.within_unique_ptr();

        let (mut drawable, mut texture, mut palette) = Default::default();
        unsafe {
            sdk::read_alt_cloth(
                raw.as_ref().unwrap(),
                &mut drawable,
                &mut texture,
                &mut palette,
            )
        }

        Ok(structs::PlayerCloth {
            drawable,
            texture,
            palette,
        })
    }

    pub fn set_clothes(
        &self,
        component: u8,
        drawable: u16,
        texture: u8,
        palette: u8,
    ) -> SomeResult<bool> {
        Ok(unsafe {
            sdk::IPlayer::SetClothes(self.raw_ptr()?, component, drawable, texture, palette)
        })
    }

    pub fn get_dlc_clothes(&self, component: u8) -> SomeResult<structs::PlayerDlcCloth> {
        let raw =
            unsafe { sdk::IPlayer::GetDlcClothes(self.raw_ptr()?, component) }.within_unique_ptr();

        let (mut drawable, mut texture, mut palette, mut dlc) = Default::default();

        unsafe {
            sdk::read_alt_dlc_cloth(
                raw.as_ref().unwrap(),
                &mut drawable,
                &mut texture,
                &mut palette,
                &mut dlc,
            )
        }

        Ok(structs::PlayerDlcCloth {
            drawable,
            texture,
            palette,
            dlc,
        })
    }

    pub fn set_dlc_clothes(
        &self,
        component: u8,
        drawable: u16,
        texture: u8,
        palette: u8,
        dlc: impl IntoHash,
    ) -> SomeResult<bool> {
        Ok(unsafe {
            sdk::IPlayer::SetDlcClothes(
                self.raw_ptr()?,
                component,
                drawable,
                texture,
                palette,
                dlc.into_hash(),
            )
        })
    }

    pub fn get_prop(&self, component: u8) -> SomeResult<structs::PlayerProp> {
        let raw = unsafe { sdk::IPlayer::GetProps(self.raw_ptr()?, component) }.within_unique_ptr();

        let (mut drawable, mut texture) = Default::default();

        unsafe {
            sdk::read_alt_prop(
                raw.as_ref().unwrap(),
                &mut drawable as *mut u16,
                &mut texture as *mut u8,
            )
        }

        Ok(structs::PlayerProp { drawable, texture })
    }

    pub fn set_prop(&self, component: u8, drawable: u16, texture: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::SetProps(self.raw_ptr()?, component, drawable, texture) })
    }

    pub fn get_dlc_prop(&self, component: u8) -> SomeResult<structs::PlayerDlcProp> {
        let raw =
            unsafe { sdk::IPlayer::GetDlcProps(self.raw_ptr()?, component) }.within_unique_ptr();

        let (mut drawable, mut texture, mut dlc) = Default::default();
        unsafe {
            sdk::read_alt_dlc_prop(raw.as_ref().unwrap(), &mut drawable, &mut texture, &mut dlc)
        }

        Ok(structs::PlayerDlcProp {
            drawable,
            texture,
            dlc,
        })
    }

    pub fn set_dlc_prop(
        &self,
        component: u8,
        drawable: u8,
        texture: u8,
        dlc: impl IntoHash,
    ) -> SomeResult<bool> {
        Ok(unsafe {
            sdk::IPlayer::SetDlcProps(
                self.raw_ptr()?,
                component,
                drawable,
                texture,
                dlc.into_hash(),
            )
        })
    }

    pub fn clear_props(&self, component: u8) -> VoidResult {
        unsafe { sdk::IPlayer::ClearProps(self.raw_ptr()?, component) }
        Ok(())
    }

    pub fn is_entity_in_streaming_range(&self, entity_id: SyncId) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsEntityInStreamingRange(self.raw_ptr()?, entity_id) })
    }

    pub fn invincible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::GetInvincible(self.raw_ptr()?) })
    }

    pub fn set_invincible(&self, toggle: bool) -> VoidResult {
        unsafe { sdk::IPlayer::SetInvincible(self.raw_ptr()?, toggle) }
        Ok(())
    }

    pub fn set_into_vehicle(
        &self,
        vehicle: impl Into<vehicle::VehicleContainer>,
        seat: u8,
    ) -> VoidResult {
        let vehicle: vehicle::VehicleContainer = vehicle.into();
        unsafe { sdk::IPlayer::SetIntoVehicle(self.raw_ptr()?, vehicle.raw_ptr()?, seat) }
        Ok(())
    }

    pub fn play_ambient_speech(
        &self,
        speech_name: impl ToString,
        speech_param: impl ToString,
        speech_dict_hash: Hash,
    ) -> VoidResult {
        unsafe {
            sdk::IPlayer::PlayAmbientSpeech(
                self.raw_ptr()?,
                speech_name.to_string(),
                speech_param.to_string(),
                speech_dict_hash,
            )
        }
        Ok(())
    }

    pub fn set_head_overlay(&self, overlay_id: u8, index: u8, opacity: f32) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::SetHeadOverlay(self.raw_ptr()?, overlay_id, index, opacity) })
    }

    pub fn get_head_overlay(&self, overlay_id: u8) -> SomeResult<structs::PlayerHeadOverlay> {
        let raw = unsafe { sdk::IPlayer::GetHeadOverlay(self.raw_ptr()?, overlay_id) }
            .within_unique_ptr();

        let (mut index, mut opacity, mut color_type, mut color_index, mut second_color_index) =
            Default::default();
        unsafe {
            sdk::read_alt_head_overlay(
                raw.as_ref().unwrap(),
                &mut index,
                &mut opacity,
                &mut color_type,
                &mut color_index,
                &mut second_color_index,
            )
        }

        Ok(structs::PlayerHeadOverlay {
            index,
            opacity,
            color_type,
            color_index,
            second_color_index,
        })
    }

    pub fn remove_head_overlay(&self, overlay_id: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::RemoveHeadOverlay(self.raw_ptr()?, overlay_id) })
    }

    pub fn set_head_overlay_color(
        &self,
        overlay_id: u8,
        color_type: u8,
        color_index: u8,
        second_color_index: u8,
    ) -> SomeResult<bool> {
        Ok(unsafe {
            sdk::IPlayer::SetHeadOverlayColor(
                self.raw_ptr()?,
                overlay_id,
                color_type,
                color_index,
                second_color_index,
            )
        })
    }

    pub fn set_face_feature(&self, index: u8, scale: f32) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::SetFaceFeature(self.raw_ptr()?, index, scale) })
    }

    pub fn get_face_feature_scale(&self, index: u8) -> SomeResult<f32> {
        Ok(unsafe { sdk::IPlayer::GetFaceFeatureScale(self.raw_ptr()?, index) })
    }

    pub fn remove_face_feature(&self, index: u8) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::RemoveFaceFeature(self.raw_ptr()?, index) })
    }

    pub fn set_head_blend_palette_color(
        &self,
        id: u8,
        red: u8,
        green: u8,
        blue: u8,
    ) -> SomeResult<bool> {
        Ok(
            unsafe {
                sdk::IPlayer::SetHeadBlendPaletteColor(self.raw_ptr()?, id, red, green, blue)
            },
        )
    }

    pub fn get_head_blend_palette_color(&self, id: u8) -> SomeResult<Rgba> {
        Ok(helpers::read_cpp_rgba(
            unsafe { sdk::IPlayer::GetHeadBlendPaletteColor(self.raw_ptr()?, id) }
                .within_unique_ptr(),
        ))
    }

    pub fn set_eye_color(&self, color: i16) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::SetEyeColor(self.raw_ptr()?, color) })
    }

    pub fn get_eye_color(&self) -> SomeResult<i16> {
        Ok(unsafe { sdk::IPlayer::GetEyeColor(self.raw_ptr()?) })
    }

    pub fn set_hair_color(&self, color: u8) -> VoidResult {
        unsafe { sdk::IPlayer::SetHairColor(self.raw_ptr()?, color) }
        Ok(())
    }

    pub fn get_hair_color(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IPlayer::GetHairColor(self.raw_ptr()?) })
    }

    pub fn set_hair_highlight_color(&self, color: u8) -> VoidResult {
        unsafe { sdk::IPlayer::SetHairHighlightColor(self.raw_ptr()?, color) }
        Ok(())
    }

    pub fn get_hair_highlight_color(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IPlayer::GetHairHighlightColor(self.raw_ptr()?) })
    }

    pub fn interior_location(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IPlayer::GetInteriorLocation(self.raw_ptr()?) })
    }

    pub fn last_damaged_body_part(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IPlayer::GetLastDamagedBodyPart(self.raw_ptr()?) })
    }

    pub fn set_last_damaged_body_part(&self, part: u32) -> VoidResult {
        unsafe { sdk::IPlayer::SetLastDamagedBodyPart(self.raw_ptr()?, part) }
        Ok(())
    }

    pub fn send_names(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::GetSendNames(self.raw_ptr()?) })
    }

    pub fn set_send_names(&self, toggle: bool) -> VoidResult {
        unsafe { sdk::IPlayer::SetSendNames(self.raw_ptr()?, toggle) }
        Ok(())
    }

    /// ### Examples
    /// Using [default options](struct.PlayAnimation.html#impl-Default-for-PlayAnimation).
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::BaseObjectPoolFuncs;
    /// # fn test() -> altv::VoidResult {
    /// let player = altv::Player::all()[0].clone();
    ///
    /// player.play_animation("cellphone@", "cellphone_text_in", Default::default())?;
    /// # Ok(()) }
    /// ```
    /// Custom flags.
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::BaseObjectPoolFuncs;
    /// # fn test() -> altv::VoidResult {
    /// let player = altv::Player::all()[0].clone();
    ///
    /// player.play_animation(
    ///     "cellphone@",
    ///     "cellphone_text_in",
    ///     altv::PlayAnimation {
    ///         flags: altv::AnimationFlags::HoldLastFrame | altv::AnimationFlags::AbortOnWeaponDamage,
    ///         ..Default::default()
    ///     },
    /// )?;
    /// # Ok(()) }
    /// ```
    pub fn play_animation(
        &self,
        dict: impl ToString,
        name: impl ToString,
        options: structs::PlayAnimation,
    ) -> VoidResult {
        let structs::PlayAnimation {
            blend_in_speed,
            blend_out_speed,
            duration,
            flags,
            playback_rate,
            lock_x,
            lock_y,
            lock_z,
        } = options;

        unsafe {
            sdk::IPlayer::PlayAnimation(
                self.raw_ptr()?,
                dict.to_string(),
                name.to_string(),
                blend_in_speed,
                blend_out_speed,
                duration.into(),
                flags.into(),
                playback_rate,
                lock_x,
                lock_y,
                lock_z,
            )
        }

        Ok(())
    }

    pub fn play_scenario(&self, name: impl ToString) -> VoidResult {
        unsafe { sdk::IPlayer::PlayScenario(self.raw_ptr()?, name.to_string()) }
        Ok(())
    }

    pub fn emit(&self, event_name: impl ToString, args: mvalue::DynMValueArgs) -> VoidResult {
        unsafe {
            sdk::trigger_client_event(
                self.raw_ptr()?,
                event_name.to_string(),
                mvalue::serialize_args(args)?,
            );
        }
        Ok(())
    }

    pub fn emit_unreliable(
        &self,
        event_name: impl ToString,
        args: mvalue::DynMValueArgs,
    ) -> VoidResult {
        unsafe {
            sdk::trigger_client_event_unreliable(
                self.raw_ptr()?,
                event_name.to_string(),
                mvalue::serialize_args(args)?,
            );
        }
        Ok(())
    }

    pub fn streamed_entities(&self) -> SomeResult<Vec<(AnyEntity, SquaredDistance)>> {
        let raw_entities = unsafe { sdk::IPlayer::GetStreamedEntities(self.raw_ptr()?) };
        let mut entities = Vec::new();

        for e in raw_entities.into_iter() {
            let entity = {
                let raw_ptr = unsafe { sdk::read_streamed_entity_key(e) };
                let entity =
                    Resource::with(|resource| helpers::get_entity_by_ptr(raw_ptr, resource));
                match entity {
                    Some(e) => e,
                    None => {
                        anyhow::bail!("Failed to get streamed entity by ptr: {raw_ptr:?}")
                    }
                }
            };
            let squared_distance = unsafe { sdk::read_streamed_entity_value(e) };

            entities.push((entity, squared_distance));
        }

        Ok(entities)
    }

    /// Returns current ammo value of specified ammo type.
    /// If player does not have ammo of specified ammo type, 0 will be returned.
    ///
    /// # Examples
    ///
    /// Passing ammo type as string.
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::BaseObjectPoolFuncs;
    /// # fn test() -> altv::VoidResult {
    /// let player = altv::Player::all()[0].clone();
    ///
    /// let ammo: u16 = player.get_ammo("ammo_pistol")?;
    /// # Ok(()) }
    /// ```
    ///
    /// Using AmmoType enum.
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::BaseObjectPoolFuncs;
    /// # fn test() -> altv::VoidResult {
    /// let player = altv::Player::all()[0].clone();
    ///
    /// let ammo: u16 = player.get_ammo(altv::AmmoType::Pistol)?;
    /// # Ok(()) }
    /// ```
    pub fn get_ammo(&self, ammo_type_hash: impl IntoHash) -> SomeResult<u16> {
        Ok(unsafe { sdk::IPlayer::GetAmmo(self.raw_ptr()?, ammo_type_hash.into_hash()) })
    }

    /// Sets ammo value of specified ammo type.
    ///
    /// # Examples
    ///
    /// Passing ammo type as string.
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::BaseObjectPoolFuncs;
    /// # fn test() -> altv::VoidResult {
    /// let player = altv::Player::all()[0].clone();
    ///
    /// player.set_ammo("ammo_pistol", 10)?;
    /// # Ok(()) }
    /// ```
    ///
    /// Using AmmoType enum.
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::BaseObjectPoolFuncs;
    /// # fn test() -> altv::VoidResult {
    /// let player = altv::Player::all()[0].clone();
    ///
    /// player.set_ammo(altv::AmmoType::Pistol, 10)?;
    /// # Ok(()) }
    /// ```
    pub fn set_ammo(&self, ammo_type_hash: impl IntoHash, ammo: u16) -> VoidResult {
        unsafe { sdk::IPlayer::SetAmmo(self.raw_ptr()?, ammo_type_hash.into_hash(), ammo) }
        Ok(())
    }

    /// Returns current ammo value of specified weapon.
    /// If player does not have this weapon, 0 will be returned.
    ///
    /// # Examples
    ///
    /// Passing weapon as string.
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::BaseObjectPoolFuncs;
    /// # fn test() -> altv::VoidResult {
    /// let player = altv::Player::all()[0].clone();
    ///
    /// let ammo: u16 = player.get_weapon_ammo("weapon_pistol")?;
    /// # Ok(()) }
    /// ```
    pub fn get_weapon_ammo(&self, weapon_hash: impl IntoHash) -> SomeResult<u16> {
        Ok(unsafe { sdk::IPlayer::GetWeaponAmmo(self.raw_ptr()?, weapon_hash.into_hash()) })
    }

    pub fn set_weapon_ammo(&self, weapon_hash: impl IntoHash, ammo: u16) -> VoidResult {
        unsafe { sdk::IPlayer::SetWeaponAmmo(self.raw_ptr()?, weapon_hash.into_hash(), ammo) }
        Ok(())
    }

    pub fn has_weapon(&self, weapon_hash: impl IntoHash) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::HasWeapon(self.raw_ptr()?, weapon_hash.into_hash()) })
    }

    pub fn get_ammo_special_type(
        &self,
        ammo_type_hash: impl IntoHash,
    ) -> SomeResult<altv_sdk::AmmoSpecialType> {
        let raw = unsafe {
            sdk::IPlayer::GetAmmoSpecialType(self.raw_ptr()?, ammo_type_hash.into_hash())
        };
        Ok(altv_sdk::AmmoSpecialType::try_from(raw).unwrap())
    }

    pub fn set_ammo_special_type(
        &self,
        ammo_type_hash: impl IntoHash,
        ammo_special_type: altv_sdk::AmmoSpecialType,
    ) -> VoidResult {
        unsafe {
            sdk::IPlayer::SetAmmoSpecialType(
                self.raw_ptr()?,
                ammo_type_hash.into_hash(),
                ammo_special_type as u32,
            )
        }
        Ok(())
    }

    pub fn get_ammo_flags(&self, ammo_type_hash: impl IntoHash) -> SomeResult<structs::AmmoFlags> {
        let ptr =
            unsafe { sdk::IPlayer::GetAmmoFlags(self.raw_ptr()?, ammo_type_hash.into_hash()) }
                .within_unique_ptr();
        Ok(structs::AmmoFlags::new(ptr))
    }

    pub fn set_ammo_flags(
        &self,
        ammo_type_hash: impl IntoHash,
        ammo_flags: structs::AmmoFlags,
    ) -> VoidResult {
        unsafe {
            sdk::IPlayer::SetAmmoFlags(
                self.raw_ptr()?,
                ammo_type_hash.into_hash(),
                ammo_flags.infinite_ammo,
                ammo_flags.add_smoke_on_explosion,
                ammo_flags.fuse,
                ammo_flags.fixed_after_explosion,
            )
        }
        Ok(())
    }

    pub fn set_ammo_max100(&self, ammo_type_hash: impl IntoHash, ammo: i32) -> VoidResult {
        unsafe { sdk::IPlayer::SetAmmoMax100(self.raw_ptr()?, ammo_type_hash.into_hash(), ammo) }
        Ok(())
    }

    pub fn get_ammo_max(&self, ammo_type_hash: impl IntoHash) -> SomeResult<i32> {
        Ok(unsafe { sdk::IPlayer::GetAmmoMax(self.raw_ptr()?, ammo_type_hash.into_hash()) })
    }

    pub fn set_ammo_max(&self, ammo_type_hash: impl IntoHash, ammo: i32) -> VoidResult {
        unsafe { sdk::IPlayer::SetAmmoMax(self.raw_ptr()?, ammo_type_hash.into_hash(), ammo) }
        Ok(())
    }

    pub fn get_ammo_max50(&self, ammo_type_hash: impl IntoHash) -> SomeResult<i32> {
        Ok(unsafe { sdk::IPlayer::GetAmmoMax50(self.raw_ptr()?, ammo_type_hash.into_hash()) })
    }

    pub fn set_ammo_max50(&self, ammo_type_hash: impl IntoHash, ammo: i32) -> VoidResult {
        unsafe { sdk::IPlayer::SetAmmoMax50(self.raw_ptr()?, ammo_type_hash.into_hash(), ammo) }
        Ok(())
    }

    pub fn get_ammo_max100(&self, ammo_type_hash: impl IntoHash) -> SomeResult<i32> {
        Ok(unsafe { sdk::IPlayer::GetAmmoMax100(self.raw_ptr()?, ammo_type_hash.into_hash()) })
    }

    pub fn decorations(&self) -> SomeResult<Vec<structs::Decoration>> {
        Ok(unsafe {
            sdk::IPlayer::GetDecorations(self.raw_ptr()?)
                .into_iter()
                .map(|v| {
                    let (mut collection, mut overlay) = Default::default();
                    sdk::read_alt_decoration(v, &mut collection, &mut overlay);

                    structs::Decoration {
                        collection,
                        overlay,
                    }
                })
                .collect()
        })
    }

    pub fn add_decoration(&self, collection: impl IntoHash, overlay: impl IntoHash) -> VoidResult {
        unsafe {
            sdk::IPlayer::AddDecoration(
                self.raw_ptr()?,
                collection.into_hash(),
                overlay.into_hash(),
            )
        }
        Ok(())
    }

    pub fn remove_decoration(
        &self,
        collection: impl IntoHash,
        overlay: impl IntoHash,
    ) -> VoidResult {
        unsafe {
            sdk::IPlayer::RemoveDecoration(
                self.raw_ptr()?,
                collection.into_hash(),
                overlay.into_hash(),
            )
        }
        Ok(())
    }

    pub fn clear_decorations(&self) -> VoidResult {
        unsafe { sdk::IPlayer::ClearDecorations(self.raw_ptr()?) }
        Ok(())
    }

    pub fn net_ownership_disabled(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IPlayer::IsNetworkOwnershipDisabled(self.raw_ptr()?) })
    }

    pub fn disable_net_ownership(&self, disable: bool) -> VoidResult {
        unsafe { sdk::IPlayer::SetNetworkOwnershipDisabled(self.raw_ptr()?, disable) }
        Ok(())
    }

    pub fn cloud_id(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IPlayer::GetCloudID(self.raw_ptr()?) }.to_string())
    }

    pub fn cloud_auth_result(&self) -> SomeResult<altv_sdk::CloudAuthResult> {
        Ok(altv_sdk::CloudAuthResult::try_from(unsafe {
            sdk::IPlayer::GetCloudAuthResult(self.raw_ptr()?)
        })
        .unwrap())
    }

    pub fn get_blood_damage_base64(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IPlayer::GetBloodDamageBase64(self.raw_ptr()?) }.to_string())
    }

    pub fn set_blood_damage_base64(&self, blood_damage: impl ToString) -> VoidResult {
        unsafe { sdk::IPlayer::SetBloodDamageBase64(self.raw_ptr()?, blood_damage.to_string()) }
        Ok(())
    }
}

impl StreamSyncedEntityMeta for player::Player {}
impl LocalPlayerMeta for player::Player {}
