use crate::{
    base_objects::{
        col_shape,
        extra_pools::{AnyEntity, SyncId},
        inherit_ptrs, BaseObjectInheritPtrs,
    },
    helpers, sdk,
    vector::{Vector2, Vector3},
    SomeResult, VoidResult,
};
use autocxx::prelude::*;

pub type ColShapeRawPtr = *mut col_shape::ColShapeStruct;

/// # **`ColShape implementation`**
impl col_shape::ColShape {
    pub fn new_circle(pos: impl Into<Vector2>, radius: f32) -> col_shape::ColShapeContainer {
        let pos = pos.into();

        helpers::create_base_object!(
            col_shape,
            sdk::ICore::CreateColShapeCircle(pos.x(), pos.y(), 0.0, radius),
            panic!("Failed to create ColShapeCircle")
        )
    }

    pub fn new_sphere(pos: impl Into<Vector3>, radius: f32) -> col_shape::ColShapeContainer {
        let pos = pos.into();
        helpers::create_base_object!(
            col_shape,
            sdk::ICore::CreateColShapeSphere(pos.x(), pos.y(), pos.z(), radius),
            panic!("Failed to create ColShapeSphere")
        )
    }

    pub fn new_rectangle(
        first_point: impl Into<Vector2>,
        second_point: impl Into<Vector2>,
    ) -> col_shape::ColShapeContainer {
        let first_point = first_point.into();
        let second_point = second_point.into();

        helpers::create_base_object!(
            col_shape,
            sdk::ICore::CreateColShapeRectangle(
                first_point.x(),
                first_point.y(),
                second_point.x(),
                second_point.y(),
                0.0, // don't ask me what is it
            ),
            panic!("Failed to create ColShapeRectangle")
        )
    }

    pub fn new_cuboid(
        first_point: impl Into<Vector3>,
        second_point: impl Into<Vector3>,
    ) -> col_shape::ColShapeContainer {
        let first_point = first_point.into();
        let second_point = second_point.into();

        helpers::create_base_object!(
            col_shape,
            sdk::ICore::CreateColShapeCube(
                first_point.x(),
                first_point.y(),
                first_point.z(),
                second_point.x(),
                second_point.y(),
                second_point.z(),
            ),
            panic!("Failed to create ColShapeCube")
        )
    }

    pub fn new_cylinder(
        pos: impl Into<Vector3>,
        radius: f32,
        height: f32,
    ) -> col_shape::ColShapeContainer {
        let pos = pos.into();

        helpers::create_base_object!(
            col_shape,
            sdk::ICore::CreateColShapeCylinder(pos.x(), pos.y(), pos.z(), radius, height),
            panic!("Failed to create ColShapeCylinder")
        )
    }

    pub fn new_polygon(
        min_z: f32,
        max_z: f32,
        points: Vec<impl Into<Vector2>>,
    ) -> col_shape::ColShapeContainer {
        let mut cpp_points = unsafe { sdk::create_vector2_vec() }.within_unique_ptr();
        for p in points {
            let p = p.into();
            unsafe { sdk::push_to_vector2_vec(cpp_points.as_mut().unwrap(), p.x(), p.y()) };
        }

        helpers::create_base_object!(
            col_shape,
            sdk::ICore::CreateColShapePolygon(min_z, max_z, cpp_points),
            panic!("Failed to create ColShapePolygon")
        )
    }

    pub fn destroy(&self) -> VoidResult {
        col_shape::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

// intended for checkpoints & colshapes
pub trait ColShapy<InheritPtrs: inherit_ptrs::traits::ColShape>:
    BaseObjectInheritPtrs<InheritPtrs>
{
    fn raw_ptr(&self) -> SomeResult<ColShapeRawPtr> {
        Ok(self.inherit_ptrs()?.col_shape())
    }

    fn players_only(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IColShape::IsPlayersOnly(self.raw_ptr()?) })
    }

    fn set_players_only(&self, state: bool) -> VoidResult {
        unsafe { sdk::IColShape::SetPlayersOnly(self.raw_ptr()?, state) }
        Ok(())
    }

    fn is_point_in(&self, point: impl Into<Vector3>) -> SomeResult<bool> {
        let point = point.into();
        Ok(unsafe { sdk::IColShape::IsPointIn(self.raw_ptr()?, point.x(), point.y(), point.z()) })
    }

    fn is_entity_in(&self, entity: impl Into<AnyEntity>) -> SomeResult<bool> {
        Ok(unsafe { sdk::IColShape::IsEntityIn(self.raw_ptr()?, entity.into().raw_ptr()?) })
    }

    fn is_entity_id_in(&self, id: SyncId) -> SomeResult<bool> {
        Ok(unsafe { sdk::IColShape::IsEntityIdIn(self.raw_ptr()?, id) })
    }

    fn col_shape_type(&self) -> SomeResult<altv_sdk::ColShapeType> {
        let raw = unsafe { sdk::IColShape::GetColshapeType(self.raw_ptr()?) };
        Ok(altv_sdk::ColShapeType::try_from(raw).unwrap())
    }
}
