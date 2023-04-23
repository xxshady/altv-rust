use crate::{
    base_objects::checkpoint,
    helpers::{self},
    rgba::RGBA,
    sdk,
    vector::Vector3,
    world_object::WorldObject,
    SomeResult, VoidResult,
};

use autocxx::prelude::*;
use std::ptr::NonNull;

impl checkpoint::Checkpoint {
    pub fn new(
        checkpoint_type: u8,
        pos: impl Into<Vector3>,
        radius: f32,
        height: f32,
        color: impl Into<RGBA>,
    ) -> checkpoint::CheckpointContainer {
        let pos = pos.into();
        let color = color.into();

        let ptr = unsafe {
            sdk::ICore::CreateCheckpoint(
                checkpoint_type,
                pos.x(),
                pos.y(),
                pos.z(),
                radius,
                height,
                color.r(),
                color.g(),
                color.b(),
                color.a(),
            )
        };

        checkpoint::add_to_pool!(NonNull::new(ptr).unwrap())
    }

    pub fn checkpoint_type(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::ICheckpoint::GetCheckpointType(self.raw_ptr()?) })
    }

    pub fn height(&self) -> SomeResult<f32> {
        Ok(unsafe { sdk::ICheckpoint::GetHeight(self.raw_ptr()?) })
    }

    pub fn radius(&self) -> SomeResult<f32> {
        Ok(unsafe { sdk::ICheckpoint::GetRadius(self.raw_ptr()?) })
    }

    pub fn color(&self) -> SomeResult<RGBA> {
        Ok(helpers::read_cpp_rgba(
            unsafe { sdk::ICheckpoint::GetColor(self.raw_ptr()?) }.within_unique_ptr(),
        ))
    }

    pub fn next_position(&self) -> SomeResult<Vector3> {
        Ok(helpers::read_cpp_vector3(unsafe {
            sdk::ICheckpoint::GetNextPosition(self.raw_ptr()?).within_unique_ptr()
        }))
    }

    pub fn set_checkpoint_type(&self, checkpoint_type: u8) -> VoidResult {
        unsafe { sdk::ICheckpoint::SetCheckpointType(self.raw_ptr()?, checkpoint_type) };
        Ok(())
    }

    pub fn set_height(&self, height: f32) -> VoidResult {
        unsafe { sdk::ICheckpoint::SetHeight(self.raw_ptr()?, height) };
        Ok(())
    }

    pub fn set_radius(&self, radius: f32) -> VoidResult {
        unsafe { sdk::ICheckpoint::SetRadius(self.raw_ptr()?, radius) };
        Ok(())
    }

    pub fn set_color(&self, color: impl Into<RGBA>) -> VoidResult {
        let color = color.into();

        unsafe {
            sdk::ICheckpoint::SetColor(self.raw_ptr()?, color.r(), color.g(), color.b(), color.a())
        };
        Ok(())
    }

    pub fn set_next_pos(&self, pos: impl Into<Vector3>) -> VoidResult {
        let pos = pos.into();

        unsafe { sdk::ICheckpoint::SetNextPosition(self.raw_ptr()?, pos.x(), pos.y(), pos.z()) };
        Ok(())
    }

    pub fn destroy(&self) -> VoidResult {
        checkpoint::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

impl WorldObject for checkpoint::Checkpoint {}
