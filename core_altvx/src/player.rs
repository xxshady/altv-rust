use crate::{
    base_object::{
        extra_pools::{get_entity_by_id, wrappers::AnyEntity, Entity, EntityId},
        player,
    },
    world_object::WorldObject,
    SomeResult,
};

impl player::Player {
    pub fn get_by_id(id: EntityId) -> SomeResult<player::PlayerContainer> {
        get_entity_by_id!(AnyEntity::Player, id).ok_or(anyhow::anyhow!("No player with id: {id}"))
    }
}

impl WorldObject for player::Player {}
impl Entity for player::Player {}
