use super::extra_pools::EntityRawPtr;
use crate::{sdk, world_object::WorldObjectRawPtr};

pub(crate) mod traits {
    use super::*;

    pub trait WorldObject {
        fn world_object(&self) -> WorldObjectRawPtr;
    }

    pub trait Entity {
        fn entity(&self) -> EntityRawPtr;
    }
}

pub struct Entity {
    entity: EntityRawPtr,
}

impl Entity {
    pub(crate) unsafe fn new(base_raw_ptr: altv_sdk::BaseObjectRawMutPtr) -> Self {
        // TODO: null check
        Self {
            entity: sdk::base_object::to_entity(base_raw_ptr),
        }
    }
}

impl traits::Entity for Entity {
    fn entity(&self) -> EntityRawPtr {
        self.entity
    }
}

#[derive(Clone)]
pub struct WorldObject {
    world_object: WorldObjectRawPtr,
}

impl WorldObject {
    pub(crate) unsafe fn new(base_raw_ptr: altv_sdk::BaseObjectRawMutPtr) -> Self {
        // TODO: null check
        Self {
            world_object: sdk::base_object::to_world_object(base_raw_ptr),
        }
    }
}

impl traits::WorldObject for WorldObject {
    fn world_object(&self) -> WorldObjectRawPtr {
        self.world_object
    }
}

pub struct WorldEntity {
    world_object: WorldObjectRawPtr,
    entity: EntityRawPtr,
}

impl WorldEntity {
    pub(crate) unsafe fn new(base_raw_ptr: altv_sdk::BaseObjectRawMutPtr) -> Self {
        // TODO: null check
        Self {
            world_object: sdk::base_object::to_world_object(base_raw_ptr),
            entity: sdk::base_object::to_entity(base_raw_ptr),
        }
    }
}

impl traits::WorldObject for WorldEntity {
    fn world_object(&self) -> WorldObjectRawPtr {
        self.world_object
    }
}

impl traits::Entity for WorldEntity {
    fn entity(&self) -> EntityRawPtr {
        self.entity
    }
}
