use crate::{
    base_objects::BasePtr, helpers::read_cpp_vector3, vector::Vector3, SomeResult, VoidResult,
};
use altv_sdk::ffi as sdk;
use autocxx::prelude::*;
use std::{fmt::Debug, ptr::NonNull};

pub type WorldObjectRawPtr = *mut sdk::alt::IWorldObject;

pub trait WorldObject: BasePtr {
    fn ptr(&self) -> SomeResult<WorldObjectRawPtr> {
        Ok(
            NonNull::new(unsafe { sdk::base_object::to_world_object(self.base_ptr()?.as_ptr()) })
                .unwrap()
                .as_ptr(),
        )
    }

    fn pos(&self) -> anyhow::Result<Vector3> {
        Ok(read_cpp_vector3(unsafe {
            sdk::IWorldObject::GetPosition(self.ptr()?).within_unique_ptr()
        }))
    }

    fn set_pos(&self, pos: Vector3) -> VoidResult {
        unsafe { sdk::IWorldObject::SetPosition(self.ptr()?, pos.x(), pos.y(), pos.z()) }
        Ok(())
    }

    fn dimension(&self) -> anyhow::Result<i32> {
        Ok(unsafe { sdk::IWorldObject::GetDimension(self.ptr()?) })
    }

    fn set_dimension(&self, value: i32) -> VoidResult {
        unsafe { sdk::IWorldObject::SetDimension(self.ptr()?, value) }
        Ok(())
    }
}

impl Debug for dyn WorldObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dyn WorldObject <any>")
    }
}
