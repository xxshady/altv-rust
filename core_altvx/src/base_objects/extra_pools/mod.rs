mod entity;
pub(crate) use entity::{get_entity_by_id, EntityPool};
pub use entity::{Entity, EntityId, EntityRawPtr};

#[derive(Debug, Default)]
pub struct ExtraPool<T> {
    base_objects: T,
}

#[derive(Debug, Default)]
pub struct ExtraPools {
    pub entity: EntityPool,
}

pub mod wrappers {
    use crate::SomeResult;

    use super::{
        super::*,
        entity::{base_ptr_to_entity_raw_ptr, EntityRawPtr},
    };
    use player::PlayerContainer;
    use vehicle::VehicleContainer;

    #[derive(Debug)]
    pub enum AnyEntity {
        Vehicle(VehicleContainer),
        Player(PlayerContainer),
    }

    impl AnyEntity {
        pub(crate) fn raw_ptr(&self) -> SomeResult<EntityRawPtr> {
            match self {
                AnyEntity::Player(e) => base_ptr_to_entity_raw_ptr(e.base_ptr()?),
                AnyEntity::Vehicle(e) => base_ptr_to_entity_raw_ptr(e.base_ptr()?),
            }
        }
    }

    impl From<VehicleContainer> for AnyEntity {
        fn from(value: VehicleContainer) -> Self {
            AnyEntity::Vehicle(value)
        }
    }

    impl From<PlayerContainer> for AnyEntity {
        fn from(value: PlayerContainer) -> Self {
            AnyEntity::Player(value)
        }
    }
}
