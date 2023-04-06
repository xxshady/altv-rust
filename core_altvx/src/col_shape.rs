use crate::{
    base_objects::{col_shape, extra_pools::wrappers::IntoAnyEntity},
    exports::EntityId,
    sdk,
    vector::{IntoVector2, IntoVector3},
    world_object::WorldObject,
    SomeResult, VoidResult,
};
use autocxx::prelude::*;
use std::ptr::NonNull;

impl col_shape::ColShape {
    pub fn new_circle(pos: impl IntoVector2, radius: f32) -> col_shape::ColShapeContainer {
        let pos = pos.into_vector2();
        let ptr = unsafe { sdk::ICore::CreateColShapeCircle(pos.x(), pos.y(), 0.0, radius) };
        col_shape::add_to_pool!(NonNull::new(ptr).unwrap())
    }

    pub fn new_sphere(pos: impl IntoVector3, radius: f32) -> col_shape::ColShapeContainer {
        let pos = pos.into_vector3();
        let ptr = unsafe { sdk::ICore::CreateColShapeSphere(pos.x(), pos.y(), pos.z(), radius) };
        col_shape::add_to_pool!(NonNull::new(ptr).unwrap())
    }

    pub fn new_rectangle(
        first_point: impl IntoVector2,
        second_point: impl IntoVector2,
    ) -> col_shape::ColShapeContainer {
        let first_point = first_point.into_vector2();
        let second_point = second_point.into_vector2();
        let ptr = unsafe {
            sdk::ICore::CreateColShapeRectangle(
                first_point.x(),
                first_point.y(),
                second_point.x(),
                second_point.y(),
                0.0, // don't ask me what is it
            )
        };
        col_shape::add_to_pool!(NonNull::new(ptr).unwrap())
    }

    pub fn new_cuboid(
        first_point: impl IntoVector3,
        second_point: impl IntoVector3,
    ) -> col_shape::ColShapeContainer {
        let first_point = first_point.into_vector3();
        let second_point = second_point.into_vector3();
        let ptr = unsafe {
            sdk::ICore::CreateColShapeCube(
                first_point.x(),
                first_point.y(),
                first_point.z(),
                second_point.x(),
                second_point.y(),
                second_point.z(),
            )
        };
        col_shape::add_to_pool!(NonNull::new(ptr).unwrap())
    }

    pub fn new_cylinder(
        pos: impl IntoVector3,
        radius: f32,
        height: f32,
    ) -> col_shape::ColShapeContainer {
        let pos = pos.into_vector3();
        let ptr = unsafe {
            sdk::ICore::CreateColShapeCylinder(pos.x(), pos.y(), pos.z(), radius, height)
        };
        col_shape::add_to_pool!(NonNull::new(ptr).unwrap())
    }

    pub fn new_polygon(
        min_z: f32,
        max_z: f32,
        points: Vec<impl IntoVector2>,
    ) -> col_shape::ColShapeContainer {
        let mut cpp_points = unsafe { sdk::create_vector2_vec() }.within_unique_ptr();
        for p in points {
            let p = p.into_vector2();
            unsafe { sdk::push_to_vector2_vec(cpp_points.as_mut().unwrap(), p.x(), p.y()) };
        }

        let ptr = unsafe { sdk::ICore::CreateColShapePolygon(min_z, max_z, cpp_points) };
        let ptr = NonNull::new(ptr).unwrap();
        col_shape::add_to_pool!(ptr)
    }

    pub fn players_only(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IColShape::IsPlayersOnly(self.raw_ptr()?) })
    }

    pub fn set_players_only(&self, state: bool) -> VoidResult {
        unsafe { sdk::IColShape::SetPlayersOnly(self.raw_ptr()?, state) }
        Ok(())
    }

    pub fn is_point_in(&self, point: impl IntoVector3) -> SomeResult<bool> {
        let point = point.into_vector3();
        Ok(unsafe { sdk::IColShape::IsPointIn(self.raw_ptr()?, point.x(), point.y(), point.z()) })
    }

    pub fn is_entity_in(&self, entity: impl IntoAnyEntity) -> SomeResult<bool> {
        Ok(unsafe {
            sdk::IColShape::IsEntityIn(self.raw_ptr()?, entity.into_any_entity().raw_ptr()?)
        })
    }

    pub fn is_entity_id_in(&self, id: EntityId) -> SomeResult<bool> {
        Ok(unsafe { sdk::IColShape::IsEntityIdIn(self.raw_ptr()?, id) })
    }

    // TODO: cache colshape type somehow
    pub fn col_shape_type(&self) -> SomeResult<altv_sdk::ColShapeType> {
        let raw = unsafe { sdk::IColShape::GetColshapeType(self.raw_ptr()?) };
        Ok(altv_sdk::ColShapeType::from(raw).unwrap())
    }

    pub fn destroy(&mut self) -> VoidResult {
        col_shape::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

impl WorldObject for col_shape::ColShape {}
