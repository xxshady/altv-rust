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
}

impl WorldObject for player::Player {}
impl Entity for player::Player {}

meta::impl_entity_meta_for!(StreamSyncedMeta, player::Player);
meta::impl_entity_meta_for!(SyncedMeta, player::Player);

// TODO: https://github.com/altmp/cpp-sdk/pull/73
// meta::impl_meta_type_for!(
//     LocalMeta,
//     player::Player,
//     sdk::IPlayer,
//     player::Player::raw_ptr
// );
