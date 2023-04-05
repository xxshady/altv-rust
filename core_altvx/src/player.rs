use crate::{
    base_object::{
        extra_pools::{get_entity_by_id, wrappers::AnyEntity, Entity, EntityId},
        player,
    },
    sdk,
    vector::IntoVector3,
    world_object::WorldObject,
    SomeResult, VoidResult,
};

impl player::Player {
    pub fn get_by_id(id: EntityId) -> SomeResult<player::PlayerContainer> {
        get_entity_by_id!(AnyEntity::Player, id).ok_or(anyhow::anyhow!("No player with id: {id}"))
    }

    pub fn name(&self) -> SomeResult<String> {
        Ok(unsafe { sdk::IPlayer::GetName(self.ptr()?.as_ptr()) }.to_string())
    }

    pub fn spawn(&self, model: u32, pos: impl IntoVector3) -> VoidResult {
        self.set_model(model)?;
        let pos = pos.into_vector3();
        unsafe { sdk::IPlayer::Spawn(self.ptr()?.as_ptr(), pos.x(), pos.y(), pos.z(), 0) }
        Ok(())
    }

    pub fn set_model(&self, model: u32) -> VoidResult {
        unsafe { sdk::IPlayer::SetModel(self.ptr()?.as_ptr(), model) }
        Ok(())
    }
}

impl WorldObject for player::Player {}
impl Entity for player::Player {}
