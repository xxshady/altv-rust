use crate::{
    base_objects::BasePtr, helpers::read_cpp_vector3, vector::Vector3, SomeResult, VoidResult,
};
use altv_sdk::ffi as sdk;
use autocxx::prelude::*;
use std::ptr::NonNull;

pub type WorldObjectRawPtr = *mut sdk::alt::IWorldObject;

pub trait WorldObject: BasePtr {
    fn ptr(&self) -> SomeResult<WorldObjectRawPtr> {
        Ok(
            NonNull::new(unsafe { sdk::base_object::to_world_object(self.raw_base_ptr()?) })
                .unwrap()
                .as_ptr(),
        )
    }

    fn pos(&self) -> SomeResult<Vector3> {
        Ok(read_cpp_vector3(unsafe {
            sdk::IWorldObject::GetPosition(self.ptr()?).within_unique_ptr()
        }))
    }

    fn set_pos(&self, pos: impl Into<Vector3>) -> VoidResult {
        let pos = pos.into();
        unsafe { sdk::IWorldObject::SetPosition(self.ptr()?, pos.x(), pos.y(), pos.z()) }
        Ok(())
    }

    fn dimension(&self) -> SomeResult<i32> {
        Ok(unsafe { sdk::IWorldObject::GetDimension(self.ptr()?) })
    }

    fn set_dimension(&self, value: i32) -> VoidResult {
        unsafe { sdk::IWorldObject::SetDimension(self.ptr()?, value) }
        Ok(())
    }
}
