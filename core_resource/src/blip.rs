use crate::{
    base_objects::blip,
    sdk,
    world_object::WorldObject,
    VoidResult,
};
use std::ptr::NonNull;

impl blip::Blip {
    pub fn new_point(x: f32, y: f32, z: f32) -> blip::BlipContainer {
        let ptr = unsafe { sdk::ICore::CreateBlip(std::ptr::null_mut(), 4, x, y, z) };
        blip::add_to_pool!(NonNull::new(ptr).unwrap());
    }

    pub fn destroy(&self) -> VoidResult {
        blip::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

impl WorldObject for blip::Blip {}
