use crate::{sdk, base_objects::col_shape_circle, helpers, vector::Vector2, VoidResult, SomeResult};

/// # **`ColShapeCircle implementation`**
impl col_shape_circle::ColShapeCircle {
    pub fn new(pos: impl Into<Vector2>, radius: f32) -> col_shape_circle::ColShapeCircleContainer {
        let pos = pos.into();

        helpers::create_base_object!(
            col_shape_circle,
            sdk::col_shape::to_col_shape_circle(sdk::ICore::CreateColShapeCircle(
                pos.x(),
                pos.y(),
                0.0,
                radius
            )),
            panic!("Failed to create ColShapeCircle")
        )
    }

    pub fn destroy(&self) -> VoidResult {
        col_shape_circle::remove_from_pool!(self)?;
        self.internal_destroy()
    }

    pub fn radius(&self) -> SomeResult<f32> {
        Ok(unsafe { sdk::IColShapeCircle::GetRadius(self.raw_ptr()?) })
    }
}
