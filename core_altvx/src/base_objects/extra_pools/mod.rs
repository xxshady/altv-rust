mod entity;
use entity::EntityPool;

pub(crate) use entity::get_entity_by_id;
pub use entity::Entity;
pub use entity::EntityId;
pub use entity::EntityRawPtr;

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
                AnyEntity::Player(e) => base_ptr_to_entity_raw_ptr(e.try_borrow()?.base_ptr()?),
                AnyEntity::Vehicle(e) => base_ptr_to_entity_raw_ptr(e.try_borrow()?.base_ptr()?),
            }
        }
    }

    pub trait IntoAnyEntity {
        fn into_any_entity(self) -> AnyEntity;
    }

    impl IntoAnyEntity for AnyEntity {
        fn into_any_entity(self) -> AnyEntity {
            self
        }
    }
}
