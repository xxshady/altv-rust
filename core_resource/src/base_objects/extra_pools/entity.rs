use autocxx::prelude::*;
use std::collections::HashMap;

use super::{AnyEntity, ExtraPool};
use crate::{
    base_objects::{inherit_ptrs, player, BaseObjectInheritPtrs},
    helpers::{self, read_cpp_vector3, Hash},
    sdk, structs,
    vector::Vector3,
    SomeResult, VoidResult,
};

pub type SyncId = u16;
pub type EntityPool = ExtraPool<HashMap<u32, AnyEntity>>;
pub type EntityRawPtr = *mut sdk::alt::IEntity;

pub trait Entity<InheritPtrs: inherit_ptrs::traits::Entity>:
    BaseObjectInheritPtrs<InheritPtrs>
{
    fn raw_ptr(&self) -> SomeResult<EntityRawPtr> {
        Ok(self.inherit_ptrs()?.entity())
    }

    fn sync_id(&self) -> SomeResult<SyncId> {
        Ok(unsafe { sdk::IEntity::GetSyncID(self.raw_ptr()?) })
    }

    fn model(&self) -> SomeResult<Hash> {
        Ok(unsafe { sdk::IEntity::GetModel(self.raw_ptr()?) })
    }

    fn attach_to_entity_bone_index(
        &self,
        entity: impl Into<AnyEntity>,
        attach: structs::AttachToEntityBoneIndex,
    ) -> VoidResult {
        let structs::AttachToEntityBoneIndex {
            other_bone_index,
            my_bone_index,
            pos,
            rot,
            collision,
            no_fixed_rotation,
        } = attach;

        let entity = entity.into();
        unsafe {
            sdk::IEntity::AttachToEntity(
                self.raw_ptr()?,
                entity.raw_ptr()?,
                other_bone_index,
                my_bone_index,
                pos.x(),
                pos.y(),
                pos.z(),
                rot.x(),
                rot.y(),
                rot.z(),
                collision,
                no_fixed_rotation,
            )
        }
        Ok(())
    }

    fn attach_to_entity_bone_name(
        &self,
        entity: impl Into<AnyEntity>,
        attach: structs::AttachToEntityBoneName,
    ) -> VoidResult {
        let structs::AttachToEntityBoneName {
            other_bone_name,
            my_bone_name,
            pos,
            rot,
            collision,
            no_fixed_rotation,
        } = attach;

        let entity = entity.into();
        unsafe {
            sdk::IEntity::AttachToEntity1(
                self.raw_ptr()?,
                entity.raw_ptr()?,
                other_bone_name,
                my_bone_name,
                pos.x(),
                pos.y(),
                pos.z(),
                rot.x(),
                rot.y(),
                rot.z(),
                collision,
                no_fixed_rotation,
            )
        }
        Ok(())
    }

    fn detach(&self) -> VoidResult {
        unsafe {
            sdk::IEntity::Detach(self.raw_ptr()?);
        }
        Ok(())
    }

    fn net_owner(&self) -> SomeResult<Option<player::PlayerContainer>> {
        helpers::get_any_option_base_object!(sdk::IEntity::GetNetworkOwner(self.raw_ptr()?), player)
    }

    fn set_net_owner(&self, owner: player::PlayerContainer, disable_migration: bool) -> VoidResult {
        unsafe {
            sdk::IEntity::SetNetworkOwner(self.raw_ptr()?, owner.raw_ptr()?, disable_migration);
        }
        Ok(())
    }

    fn rot(&self) -> SomeResult<Vector3> {
        let raw = unsafe { sdk::IEntity::GetRotation(self.raw_ptr()?) }.within_unique_ptr();
        Ok(read_cpp_vector3(raw))
    }

    fn set_rot(&self, rot: impl Into<Vector3>) -> VoidResult {
        let rot = rot.into();
        unsafe {
            sdk::IEntity::SetRotation(self.raw_ptr()?, rot.x(), rot.y(), rot.z());
        }
        Ok(())
    }

    fn streamed(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IEntity::GetStreamed(self.raw_ptr()?) })
    }

    fn set_streamed(&self, toggle: bool) -> VoidResult {
        unsafe {
            sdk::IEntity::SetStreamed(self.raw_ptr()?, toggle);
        }
        Ok(())
    }

    fn visible(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IEntity::GetVisible(self.raw_ptr()?) })
    }

    fn set_visible(&self, toggle: bool) -> VoidResult {
        unsafe {
            sdk::IEntity::SetVisible(self.raw_ptr()?, toggle);
        }
        Ok(())
    }

    fn frozen(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IEntity::IsFrozen(self.raw_ptr()?) })
    }

    fn set_frozen(&self, toggle: bool) -> VoidResult {
        unsafe {
            sdk::IEntity::SetFrozen(self.raw_ptr()?, toggle);
        }
        Ok(())
    }

    fn collision(&self) -> SomeResult<bool> {
        Ok(unsafe { sdk::IEntity::HasCollision(self.raw_ptr()?) })
    }

    fn set_collision(&self, toggle: bool) -> VoidResult {
        unsafe {
            sdk::IEntity::SetCollision(self.raw_ptr()?, toggle);
        }
        Ok(())
    }
}

impl EntityPool {
    pub fn add(&mut self, entity: AnyEntity) {
        let id = match &entity {
            AnyEntity::Player(p) => p.id(),
            AnyEntity::Vehicle(v) => v.id(),
            AnyEntity::Ped(v) => v.id(),
            AnyEntity::NetworkObject(v) => v.id(),
        }
        .unwrap();
        logger::debug!("add entity id: {id}");

        self.base_objects.insert(id, entity);
    }

    pub fn remove(&mut self, base_object: altv_sdk::BaseObjectMutPtr) {
        // why? because i dont know how to borrow base object container immutably to call .id()
        // when its mutably borrowed for .destroy() method
        let id = unsafe { sdk::IBaseObject::GetID(base_object.as_ptr()) };
        logger::debug!("remove entity id: {id}");

        self.base_objects.remove(&id);
    }

    pub fn get_by_id(&self, id: u32) -> Option<&AnyEntity> {
        self.base_objects.get(&id)
    }
}

#[macro_export]
macro_rules! __get_entity_by_id {
    ($entity_type:path, $id:expr) => {
        $crate::resource::Resource::with_extra_base_object_pools_ref(|v, _| {
            match v.entity.get_by_id($id) {
                Some(_wrapper @ $entity_type(entity)) => Some(entity.clone()),
                None | Some(_) => None,
            }
        })
    };
}

pub(crate) use __get_entity_by_id as get_entity_by_id;
