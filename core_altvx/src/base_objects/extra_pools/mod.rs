mod entity;
use entity::EntityPool;

pub(crate) use entity::get_entity_by_id;
pub use entity::Entity;
pub use entity::EntityId;

#[derive(Debug, Default)]
pub struct ExtraPool<T> {
    base_objects: T,
}

#[derive(Debug, Default)]
pub struct ExtraPools {
    pub entity: EntityPool,
}

pub mod wrappers {
    use super::super::*;
    use player::PlayerContainer;
    use vehicle::VehicleContainer;

    #[derive(Debug)]
    pub enum AnyEntity {
        Vehicle(VehicleContainer),
        Player(PlayerContainer),
    }
}
