use crate::{
    base_objects::{
        extra_pools::{get_entity_by_id, wrappers::AnyEntity, Entity, EntityId},
        meta, player,
    },
    helpers::IntoHash,
    resource::Resource,
    sdk, structs,
    vector::IntoVector3,
    world_object::WorldObject,
    SomeResult, VoidResult,
};
use autocxx::prelude::*;

impl player::Player {
    pub fn all() -> Vec<player::PlayerContainer> {
        Resource::with_base_objects_ref(|v, _| v.player.all())
    }

    pub fn get_by_id(id: EntityId) -> SomeResult<player::PlayerContainer> {
        get_entity_by_id!(AnyEntity::Player, id).ok_or(anyhow::anyhow!("No player with id: {id}"))
    }

    pub fn name(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IPlayer::GetName(self.raw_ptr()?) }.to_string())
    }

    pub fn spawn(&self, model: impl IntoHash, pos: impl IntoVector3) -> VoidResult {
        self.set_model(model)?;
        let pos = pos.into_vector3();
        unsafe { sdk::IPlayer::Spawn(self.raw_ptr()?, pos.x(), pos.y(), pos.z(), 0) }
        Ok(())
    }

    pub fn despawn(&self) -> VoidResult {
        unsafe { sdk::IPlayer::Despawn(self.raw_ptr()?) }
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
}

impl WorldObject for player::Player {}
impl Entity for player::Player {}

meta::impl_entity_meta_for!(StreamSyncedMeta, player::Player);
meta::impl_entity_meta_for!(SyncedMeta, player::Player);

meta::impl_meta_type_for!(
    LocalMeta,
    player::Player,
    sdk::IPlayer,
    player::Player::raw_ptr
);
