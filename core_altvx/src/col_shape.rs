pub use crate::base_object::ColShape;
use crate::{
    base_object::ColShapeContainer, resource::Resource, sdk, vector::Vector2,
    world_object::WorldObject, VoidResult,
};
use std::ptr::NonNull;

impl ColShape {
    pub fn new_circle(pos: Vector2, radius: f32) -> ColShapeContainer {
        let ptr = unsafe { sdk::ICore::CreateColShapeCircle(pos.x(), pos.y(), 0.0, radius) };
        let ptr: NonNull<sdk::alt::IColShape> = NonNull::new(ptr).unwrap();

        let container = Self::_new(
            ptr,
            NonNull::new(unsafe { sdk::col_shape::to_base_object(ptr.as_ptr()) }).unwrap(),
        );

        Resource::with_base_objects_mut(|mut v, _| {
            v.create_col_shape(ptr, container.clone());
        });

        container
    }

    pub fn destroy(&mut self) -> VoidResult {
        Resource::with_base_objects_mut(|mut v, _| -> VoidResult {
            // TODO: move it to BaseObject
            v.remove_col_shape(self.ptr()?);
            Ok(())
        })?;

        self.internal_destroy()
    }
}

impl WorldObject for ColShape {}
