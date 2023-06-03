use crate::{
    base_objects::{ped},
    helpers::{self, Hash, IntoHash},
    meta::entity_stream_synced_meta::StreamSyncedEntityMeta,
    sdk,
    vector::Vector3,
    SomeResult, VoidResult,
};

/// # **`Ped implementation`**
impl ped::Ped {
    pub fn new(
        model: impl IntoHash,
        pos: impl Into<Vector3>,
        rot: impl Into<Vector3>,
    ) -> SomeResult<ped::PedContainer> {
        let pos = pos.into();
        let rot = rot.into();

        Ok(helpers::create_base_object!(
            ped,
            sdk::ICore::CreatePed(
                model.into_hash(),
                pos.x(),
                pos.y(),
                pos.z(),
                rot.x(),
                rot.y(),
                rot.z(),
            ),
            anyhow::bail!("Ped model is incorrect or there is no free id for new entity")
        ))
    }

    pub fn destroy(&self) -> VoidResult {
        ped::remove_from_pool!(self)?;
        self.internal_destroy()
    }

    pub fn health(&self) -> SomeResult<u16> {
        Ok(unsafe { sdk::IPed::GetHealth(self.raw_ptr()?) })
    }

    pub fn set_health(&self, health: u16) -> VoidResult {
        unsafe { sdk::IPed::SetHealth(self.raw_ptr()?, health) }
        Ok(())
    }

    pub fn max_health(&self) -> SomeResult<u16> {
        Ok(unsafe { sdk::IPed::GetMaxHealth(self.raw_ptr()?) })
    }

    pub fn set_max_health(&self, max_health: u16) -> VoidResult {
        unsafe { sdk::IPed::SetMaxHealth(self.raw_ptr()?, max_health) }
        Ok(())
    }

    pub fn armour(&self) -> SomeResult<u16> {
        Ok(unsafe { sdk::IPed::GetArmour(self.raw_ptr()?) })
    }

    pub fn set_armour(&self, armour: u16) -> VoidResult {
        unsafe { sdk::IPed::SetArmour(self.raw_ptr()?, armour) }
        Ok(())
    }

    pub fn current_weapon(&self) -> SomeResult<Hash> {
        Ok(unsafe { sdk::IPed::GetCurrentWeapon(self.raw_ptr()?) })
    }

    pub fn set_current_weapon(&self, weapon: impl IntoHash) -> VoidResult {
        unsafe { sdk::IPed::SetCurrentWeapon(self.raw_ptr()?, weapon.into_hash()) }
        Ok(())
    }
}

impl StreamSyncedEntityMeta for ped::Ped {}
