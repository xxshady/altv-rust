use crate::{
    base_objects::{inherit_ptrs, BaseObjectInheritPtrs},
    helpers::read_cpp_vector3,
    vector::Vector3,
    SomeResult, VoidResult,
};
use altv_sdk::ffi as sdk;
use autocxx::prelude::*;

pub type WorldObjectRawPtr = *mut sdk::alt::IWorldObject;

pub trait WorldObject<InheritPtrs: inherit_ptrs::traits::WorldObject>:
    BaseObjectInheritPtrs<InheritPtrs>
{
    fn raw_ptr(&self) -> SomeResult<WorldObjectRawPtr> {
        Ok(self.inherit_ptrs()?.world_object())
    }

    fn pos(&self) -> SomeResult<Vector3> {
        Ok(read_cpp_vector3(unsafe {
            sdk::IWorldObject::GetPosition(self.raw_ptr()?).within_unique_ptr()
        }))
    }

    fn set_pos(&self, pos: impl Into<Vector3>) -> VoidResult {
        let pos = pos.into();
        unsafe { sdk::IWorldObject::SetPosition(self.raw_ptr()?, pos.x(), pos.y(), pos.z()) }
        Ok(())
    }

    fn dimension(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IWorldObject::GetDimension(self.raw_ptr()?) })
    }

    fn set_dimension(&self, value: i32) -> VoidResult {
        unsafe { sdk::IWorldObject::SetDimension(self.raw_ptr()?, value) }
        Ok(())
    }
}
