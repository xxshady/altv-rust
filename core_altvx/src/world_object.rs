use crate::{base_object::BaseObject, vector::Vector3};
use altv_sdk::ffi as sdk;
use autocxx::prelude::*;
use std::fmt::Debug;

pub type RawWorldObjectPointer = *mut sdk::alt::IWorldObject;

pub trait WorldObject: BaseObject {
    fn pos(&self) -> anyhow::Result<Vector3> {
        let cpp_vector = unsafe { sdk::IWorldObject::GetPosition(self.ptr().to_world_object()?) }
            .within_unique_ptr();

        let out_x = Box::into_raw(Box::new(0f32));
        let out_y = Box::into_raw(Box::new(0f32));
        let out_z = Box::into_raw(Box::new(0f32));
        unsafe {
            sdk::read_vector3(cpp_vector.as_ref().unwrap(), out_x, out_y, out_z);
            Ok(Vector3::new(*out_x, *out_y, *out_z))
        }
    }

    fn set_pos(&self, pos: Vector3) -> anyhow::Result<()> {
        Ok(unsafe {
            sdk::IWorldObject::SetPosition(self.ptr().to_world_object()?, pos.x(), pos.y(), pos.z())
        })
    }

    fn dimension(&self) -> anyhow::Result<i32> {
        Ok(unsafe { sdk::IWorldObject::GetDimension(self.ptr().to_world_object()?) })
    }

    fn set_dimension(&self, value: i32) -> anyhow::Result<()> {
        Ok(unsafe { sdk::IWorldObject::SetDimension(self.ptr().to_world_object()?, value) })
    }
}

impl Debug for dyn WorldObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dyn WorldObject <any>")
    }
}
