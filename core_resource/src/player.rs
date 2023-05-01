use crate::{
    base_objects::{
        extra_pools::{get_entity_by_id, wrappers::AnyEntity, Entity, SyncId},
        meta, player, vehicle,
    },
    helpers::{self, read_cpp_vector3, Hash, IntoHash, IntoString},
    resource::Resource,
    rgba::RGBA,
    sdk, structs,
    vector::Vector3,
    SomeResult, VoidResult,
};
use autocxx::prelude::*;

/// # **`Player implementation`**
impl player::Player {
    pub fn all() -> Vec<player::PlayerContainer> {
        Resource::with_base_objects_ref(|v, _| v.player.all())
    }

    pub fn get_by_id(id: u32) -> SomeResult<player::PlayerContainer> {
        get_entity_by_id!(AnyEntity::Player, id).ok_or(anyhow::anyhow!("No player with id: {id}"))
    }

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

    pub fn cloud_auth_hash(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IPlayer::GetCloudAuthHash(self.raw_ptr()?) }.to_string())
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

        let mut shape_first_id = 0u32;
        let mut shape_second_id = 0u32;
        let mut shape_third_id = 0u32;
        let mut skin_first_id = 0u32;
        let mut skin_second_id = 0u32;
        let mut skin_third_id = 0u32;
        let mut shape_mix = 0f32;
        let mut skin_mix = 0f32;
        let mut third_mix = 0f32;

        unsafe {
            sdk::read_alt_head_blend_data(
                raw.as_ref().unwrap(),
                &mut shape_first_id as *mut u32,
                &mut shape_second_id as *mut u32,
                &mut shape_third_id as *mut u32,
                &mut skin_first_id as *mut u32,
                &mut skin_second_id as *mut u32,
                &mut skin_third_id as *mut u32,
                &mut shape_mix as *mut f32,
                &mut skin_mix as *mut f32,
                &mut third_mix as *mut f32,
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

    pub fn remove_all_weapons(&self) -> VoidResult {
        unsafe { sdk::IPlayer::RemoveAllWeapons(self.raw_ptr()?) }
        Ok(())
    }

    pub fn weapons(&self) -> SomeResult<Vec<structs::Weapon>> {
        let raw = unsafe { sdk::IPlayer::GetWeapons(self.raw_ptr()?) };
        let weapons = raw
            .into_iter()
            .map(|v| {
                let mut hash = 0u32;
                let mut tint_index = 0u8;
                unsafe { sdk::read_weapon(v, &mut hash as *mut _, &mut tint_index as *mut _) }
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

    pub fn kick(&self, reason: impl IntoString) -> VoidResult {
        unsafe { sdk::IPlayer::Kick(self.raw_ptr()?, reason.into_string()) }
        Ok(())
    }

    pub fn get_clothes(&self, component: u8) -> SomeResult<structs::PlayerCloth> {
        let raw =
            unsafe { sdk::IPlayer::GetClothes(self.raw_ptr()?, component) }.within_unique_ptr();

        let mut drawable = 0u16;
        let mut texture = 0u8;
        let mut palette = 0u8;

        unsafe {
            sdk::read_alt_cloth(
                raw.as_ref().unwrap(),
                &mut drawable as *mut u16,
                &mut texture as *mut u8,
                &mut palette as *mut u8,
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

        let mut drawable = 0u16;
        let mut texture = 0u8;
        let mut palette = 0u8;
        let mut dlc = 0u32;

        unsafe {
            sdk::read_alt_dlc_cloth(
                raw.as_ref().unwrap(),
                &mut drawable as *mut _,
                &mut texture as *mut _,
                &mut palette as *mut _,
                &mut dlc as *mut _,
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
        dlc: u32,
    ) -> SomeResult<bool> {
        Ok(unsafe {
            sdk::IPlayer::SetDlcClothes(self.raw_ptr()?, component, drawable, texture, palette, dlc)
        })
    }

    pub fn get_prop(&self, component: u8) -> SomeResult<structs::PlayerProp> {
        let raw = unsafe { sdk::IPlayer::GetProps(self.raw_ptr()?, component) }.within_unique_ptr();

        let mut drawable = 0u16;
        let mut texture = 0u8;

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

        let mut drawable = 0u8;
        let mut texture = 0u8;
        let mut dlc = 0u32;

        unsafe {
            sdk::read_alt_dlc_prop(
                raw.as_ref().unwrap(),
                &mut drawable as *mut _,
                &mut texture as *mut _,
                &mut dlc as *mut _,
            )
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
        dlc: u32,
    ) -> SomeResult<bool> {
        Ok(
            unsafe {
                sdk::IPlayer::SetDlcProps(self.raw_ptr()?, component, drawable, texture, dlc)
            },
        )
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
        speech_name: impl IntoString,
        speech_param: impl IntoString,
        speech_dict_hash: Hash,
    ) -> VoidResult {
        unsafe {
            sdk::IPlayer::PlayAmbientSpeech(
                self.raw_ptr()?,
                speech_name.into_string(),
                speech_param.into_string(),
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

        let mut index = 0u8;
        let mut opacity = 0f32;
        let mut color_type = 0u8;
        let mut color_index = 0u8;
        let mut second_color_index = 0u8;
        unsafe {
            sdk::read_alt_head_overlay(
                raw.as_ref().unwrap(),
                &mut index as *mut _,
                &mut opacity as *mut _,
                &mut color_type as *mut _,
                &mut color_index as *mut _,
                &mut second_color_index as *mut _,
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

    pub fn get_head_blend_palette_color(&self, id: u8) -> SomeResult<RGBA> {
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
    /// Using default options
    /// ```rust
    /// player.play_animation("cellphone@", "cellphone_text_in", Default::default()).unwrap();
    /// ```
    /// Custom flags
    /// ```rust
    /// player.play_animation(
    ///     "cellphone@",
    ///     "cellphone_text_in",
    ///     altv::PlayAnimation {
    ///         flags: altv::AnimationFlags::HoldLastFrame | altv::AnimationFlags::AbortOnWeaponDamage,
    ///         ..Default::default()
    ///     },
    /// ).unwrap();
    /// ```
    pub fn play_animation(
        &self,
        dict: impl IntoString,
        name: impl IntoString,
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
                dict.into_string(),
                name.into_string(),
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
}

meta::impl_entity_meta_for!(StreamSyncedMeta, player::Player);

meta::impl_meta_type_for!(
    LocalMeta,
    player::Player,
    sdk::IPlayer,
    player::Player::raw_ptr,
);
