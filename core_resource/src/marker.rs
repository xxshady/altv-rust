use crate::{
    base_objects::{extra_pools::wrappers::AnyEntity, marker, player},
    helpers::{self},
    rgba::RGBA,
    sdk,
    vector::{Vector2, Vector3},
    world_object::WorldObject,
    SomeResult, VoidResult,
};

use crate::resource::Resource;
use autocxx::prelude::*;
use std::ptr::{null, NonNull};

impl marker::Marker {
    pub fn new_area(pos: impl Into<Vector3>, color: impl Into<RGBA>) -> marker::MarkerContainer {
        let pos = pos.into();
        let color = color.into();

        let ptr = unsafe {
            sdk::ICore::CreateMarker(
                std::ptr::null_mut(),
                1,
                pos.x(),
                pos.y(),
                pos.z(),
                color.r(),
                color.g(),
                color.b(),
                color.a(),
                std::ptr::null_mut(),
            )
        };

        marker::add_to_pool!(NonNull::new(ptr).unwrap())
    }

    pub fn destroy(&self) -> VoidResult {
        marker::remove_from_pool!(self)?;
        self.internal_destroy()
    }
}

impl WorldObject for marker::Marker {}
