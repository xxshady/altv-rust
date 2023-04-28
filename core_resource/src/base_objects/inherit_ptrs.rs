use super::extra_pools::EntityRawPtr;
use crate::{col_shape::ColShapeRawPtr, helpers, sdk, world_object::WorldObjectRawPtr};

pub(crate) mod traits {
    use super::*;

    pub trait WorldObject {
        fn world_object(&self) -> WorldObjectRawPtr;
    }

    pub trait Entity {
        fn entity(&self) -> EntityRawPtr;
    }

    pub trait ColShape {
        fn col_shape(&self) -> ColShapeRawPtr;
    }
}

#[derive(Clone)]
pub struct WorldObject {
    world_object: WorldObjectRawPtr,
}

impl WorldObject {
    pub(crate) unsafe fn new(base_raw_ptr: altv_sdk::BaseObjectRawMutPtr) -> Self {
        Self {
            world_object: helpers::base_ptr_to_raw!(base_raw_ptr, world_object),
        }
    }
}

impl traits::WorldObject for WorldObject {
    fn world_object(&self) -> WorldObjectRawPtr {
        self.world_object
    }
}

#[derive(Clone)]
pub struct WorldEntity {
    world_object: WorldObjectRawPtr,
    entity: EntityRawPtr,
}

impl WorldEntity {
    pub(crate) unsafe fn new(base_raw_ptr: altv_sdk::BaseObjectRawMutPtr) -> Self {
        Self {
            world_object: helpers::base_ptr_to_raw!(base_raw_ptr, world_object),
            entity: helpers::base_ptr_to_raw!(base_raw_ptr, entity),
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

#[derive(Clone)]
pub struct WorldColShape {
    world_object: WorldObjectRawPtr,
    col_shape: ColShapeRawPtr,
}

impl WorldColShape {
    pub(crate) unsafe fn new(base_raw_ptr: altv_sdk::BaseObjectRawMutPtr) -> Self {
        Self {
            world_object: helpers::base_ptr_to_raw!(base_raw_ptr, world_object),
            col_shape: helpers::base_ptr_to_raw!(base_raw_ptr, col_shape),
        }
    }
}

impl traits::WorldObject for WorldColShape {
    fn world_object(&self) -> WorldObjectRawPtr {
        self.world_object
    }
}

impl traits::ColShape for WorldColShape {
    fn col_shape(&self) -> ColShapeRawPtr {
        self.col_shape
    }
}
