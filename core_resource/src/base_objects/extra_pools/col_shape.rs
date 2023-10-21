use std::ptr::NonNull;

use crate::{
    base_objects::{
        extra_pools::{AnyEntity, SyncId},
        inherit_ptrs, BaseObjectInheritPtrs,
    },
    sdk,
    vector::Vector3,
    SomeResult, VoidResult,
};

pub type ColShapeRawPtr = *mut sdk::alt::IColShape;
pub type ColShapeNonNull = NonNull<sdk::alt::IColShape>;

// intended for checkpoints & colshapes
pub trait ColShape<InheritPtrs: inherit_ptrs::traits::ColShape>:
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
