use crate::{
    base_objects::object,
    helpers::{self, IntoHash},
    meta::entity_stream_synced_meta::StreamSyncedEntityMeta,
    sdk,
    vector::Vector3,
    SomeResult, VoidResult,
};

/// # **`Object implementation`**
impl object::Object {
    /// Creates new instance of Object with default params.
    /// See [object::Object::new_with_params]
    /// if you want to create network object with custom params.
    ///
    /// # Errors
    /// When the maximum number of entities is created (2^16).
    ///
    /// # Examples
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # fn test() -> altv::VoidResult {
    /// let object = altv::Object::new(
    ///    "prop_bench_04",
    ///    altv::Vector3::new(0.0, 0.0, 71.0),
    ///    altv::Vector3::new(1.0, 2.0, 3.0)
    /// )?;
    /// # Ok(()) }
    /// ```
    pub fn new(
        model: impl IntoHash,
        pos: impl Into<Vector3>,
        rot: impl Into<Vector3>,
    ) -> SomeResult<object::ObjectContainer> {
        Self::new_with_streaming_distance(model, pos, rot, 0)
    }

    pub fn new_with_streaming_distance(
        model: impl IntoHash,
        pos: impl Into<Vector3>,
        rot: impl Into<Vector3>,
        streaming_distance: u32,
    ) -> SomeResult<object::ObjectContainer> {
        Self::new_with_params(model, pos, rot, 255, 0, 100, streaming_distance)
    }

    /// Creates new instance of Object with custom params.
    ///
    /// # Errors
    /// When the maximum number of entities is created (2^16).
    ///
    /// # Examples
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # fn test() -> altv::VoidResult {
    /// let object = altv::Object::new_with_params(
    ///    "prop_bench_04",
    ///    altv::Vector3::new(0, 0, 71),
    ///    altv::Vector3::new(1.0, 2.0, 3.0),
    ///    150,
    ///    0,
    ///    999,
    ///    300,
    /// )?;
    /// # Ok(()) }
    /// ```
    pub fn new_with_params(
        model: impl IntoHash,
        pos: impl Into<Vector3>,
        rot: impl Into<Vector3>,
        alpha: u8,
        texture_variation: u8,
        lod_distance: u16,
        streaming_distance: u32,
    ) -> SomeResult<object::ObjectContainer> {
        let pos = pos.into();
        let rot = rot.into();

        Ok(helpers::create_base_object!(
            object,
            sdk::ICore::CreateObject(
                model.into_hash(),
                pos.x(),
                pos.y(),
                pos.z(),
                rot.x(),
                rot.y(),
                rot.z(),
                alpha,
                texture_variation,
                lod_distance,
                streaming_distance,
            ),
            anyhow::bail!("Object model is incorrect or there is no free id for new entity")
        ))
    }

    pub fn destroy(&self) -> VoidResult {
        object::remove_from_pool!(self)?;
        self.internal_destroy()
    }

    pub fn alpha(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IObject::GetAlpha(self.raw_ptr()?) })
    }

    pub fn set_alpha(&self, alpha: u8) -> VoidResult {
        unsafe { sdk::IObject::SetAlpha(self.raw_ptr()?, alpha) }
        Ok(())
    }

    pub fn texture_variation(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::IObject::GetTextureVariation(self.raw_ptr()?) })
    }

    pub fn set_texture_variation(&self, texture_variation: u8) -> VoidResult {
        unsafe { sdk::IObject::SetTextureVariation(self.raw_ptr()?, texture_variation) }
        Ok(())
    }

    pub fn lod_distance(&self) -> SomeResult<u16> {
        Ok(unsafe { sdk::IObject::GetLodDistance(self.raw_ptr()?) })
    }

    pub fn set_lod_distance(&self, lod_distance: u16) -> VoidResult {
        unsafe { sdk::IObject::SetLodDistance(self.raw_ptr()?, lod_distance) }
        Ok(())
    }

    pub fn activate_physics(&self) -> VoidResult {
        unsafe { sdk::IObject::ActivatePhysics(self.raw_ptr()?) }
        Ok(())
    }

    pub fn place_on_ground_properly(&self) -> VoidResult {
        unsafe { sdk::IObject::PlaceOnGroundProperly(self.raw_ptr()?) }
        Ok(())
    }
}

impl StreamSyncedEntityMeta for object::Object {}
