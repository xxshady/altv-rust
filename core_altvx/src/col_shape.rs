use crate::{
    base_objects::col_shape, sdk, vector::IntoVector2, world_object::WorldObject, VoidResult,
};
use std::ptr::NonNull;

impl col_shape::ColShape {
    pub fn new_circle(pos: impl IntoVector2, radius: f32) -> col_shape::ColShapeContainer {
        let pos = pos.into_vector2();
        let ptr = unsafe { sdk::ICore::CreateColShapeCircle(pos.x(), pos.y(), 0.0, radius) };
        col_shape::add_to_pool!(NonNull::new(ptr).unwrap())
    }

    pub fn destroy(&mut self) -> VoidResult {
        col_shape::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

impl WorldObject for col_shape::ColShape {}
