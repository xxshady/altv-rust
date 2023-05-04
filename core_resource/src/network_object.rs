use crate::{
    base_objects::{
        extra_pools::{get_entity_by_id, AnyEntity},
        network_object,
    },
    helpers::{self, IntoHash},
    meta::entity_stream_synced_meta::StreamSyncedEntityMeta,
    sdk,
    vector::Vector3,
    SomeResult, VoidResult,
};

/// # **`NetworkObject implementation`**
impl network_object::NetworkObject {
    pub fn get_by_id(id: u32) -> SomeResult<network_object::NetworkObjectContainer> {
        get_entity_by_id!(AnyEntity::NetworkObject, id)
            .ok_or(anyhow::anyhow!("No network object with id: {id}"))
    }

    /// Creates new instance of NetworkObject with default params.
    /// See [network_object::NetworkObject::new_with_params]
    /// if you want to create network object with custom params.
    ///
    /// # Errors
    /// When the maximum number of entities is created (2^16).
    ///
    /// # Examples
    /// ```rust
    /// let object = altv::NetworkObject::new("prop_bench_04", (0, 0, 71), (1.0, 2.0, 3.0)).unwrap();
    /// ```
    pub fn new(
        model: impl IntoHash,
        pos: impl Into<Vector3>,
        rot: impl Into<Vector3>,
    ) -> SomeResult<network_object::NetworkObjectContainer> {
        Self::new_with_params(model, pos, rot, 255, 0, 100)
    }

    /// Creates new instance of NetworkObject with custom params.
    ///
    /// # Errors
    /// When the maximum number of entities is created (2^16).
    ///
    /// # Examples
    /// ```rust
    /// let object = altv::NetworkObject::new_with_params(
    ///    "prop_bench_04",
    ///    (0, 0, 71),
    ///    (1.0, 2.0, 3.0),
    ///    150,
    ///    0,
    ///    999
    /// ).unwrap();
    /// ```
    pub fn new_with_params(
        model: impl IntoHash,
        pos: impl Into<Vector3>,
        rot: impl Into<Vector3>,
        alpha: u8,
        texture_variation: u8,
        lod_distance: u16,
    ) -> SomeResult<network_object::NetworkObjectContainer> {
        let pos = pos.into();
        let rot = rot.into();

        Ok(helpers::create_base_object!(
            network_object,
            sdk::ICore::CreateNetworkObject(
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
            ),
            anyhow::bail!("NetworkObject model is incorrect or there is no free id for new entity")
        ))
    }

    pub fn destroy(&self) -> VoidResult {
        network_object::remove_from_pool!(self)?;
        self.internal_destroy()
    }

    pub fn alpha(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::INetworkObject::GetAlpha(self.raw_ptr()?) })
    }

    pub fn set_alpha(&self, alpha: u8) -> VoidResult {
        unsafe { sdk::INetworkObject::SetAlpha(self.raw_ptr()?, alpha) }
        Ok(())
    }

    pub fn texture_variation(&self) -> SomeResult<u8> {
        Ok(unsafe { sdk::INetworkObject::GetTextureVariation(self.raw_ptr()?) })
    }

    pub fn set_texture_variation(&self, texture_variation: u8) -> VoidResult {
        unsafe { sdk::INetworkObject::SetTextureVariation(self.raw_ptr()?, texture_variation) }
        Ok(())
    }

    pub fn lod_distance(&self) -> SomeResult<u16> {
        Ok(unsafe { sdk::INetworkObject::GetLodDistance(self.raw_ptr()?) })
    }

    pub fn set_lod_distance(&self, lod_distance: u16) -> VoidResult {
        unsafe { sdk::INetworkObject::SetLodDistance(self.raw_ptr()?, lod_distance) }
        Ok(())
    }

    pub fn activate_physics(&self) -> VoidResult {
        unsafe { sdk::INetworkObject::ActivatePhysics(self.raw_ptr()?) }
        Ok(())
    }

    pub fn place_on_ground_properly(&self) -> VoidResult {
        unsafe { sdk::INetworkObject::PlaceOnGroundProperly(self.raw_ptr()?) }
        Ok(())
    }
}

impl StreamSyncedEntityMeta for network_object::NetworkObject {}
