use std::{collections::HashMap, ptr::NonNull};

use super::{super::BasePtr, wrappers::AnyEntity, ExtraPool};
use crate::{helpers::ModelHash, sdk, SomeResult};

pub type EntityId = u16;
pub type EntityPool = ExtraPool<HashMap<EntityId, AnyEntity>>;

pub type EntityRawPtr = *mut sdk::alt::IEntity;

pub trait Entity: BasePtr {
    fn ptr(&self) -> SomeResult<EntityRawPtr> {
        Ok(
            NonNull::new(unsafe { sdk::base_object::to_entity(self.base_ptr()?.as_ptr()) })
                .unwrap()
                .as_ptr(),
        )
    }

    fn id(&self) -> SomeResult<EntityId> {
        Ok(unsafe { sdk::IEntity::GetID(self.ptr()?) })
    }

    fn model(&self) -> SomeResult<ModelHash> {
        Ok(unsafe { sdk::IEntity::GetModel(self.ptr()?) })
    }
}

impl EntityPool {
    pub fn add(&mut self, entity: AnyEntity) {
        let id = match &entity {
            AnyEntity::Player(p) => p.borrow().id(),
            AnyEntity::Vehicle(v) => v.borrow().id(),
        };
        let id = id.unwrap();
        logger::debug!("add entity id: {id}");

        self.base_objects.insert(id, entity);
    }

    pub fn remove(&mut self, base_object: altv_sdk::BaseObjectMutPtr) {
        // why? because i dont know how to borrow base object container immutably to call .id()
        // when its mutably borrowed for .destroy() method
        let entity = unsafe { sdk::base_object::to_entity(base_object.as_ptr()) };
        let entity = NonNull::new(entity).unwrap();
        let id = unsafe { sdk::IEntity::GetID(entity.as_ptr()) };
        logger::debug!("remove entity id: {id}");

        self.base_objects.remove(&id);
    }

    pub fn get_by_id(&self, id: EntityId) -> Option<&AnyEntity> {
        self.base_objects.get(&id)
    }
}

#[macro_export]
macro_rules! __get_entity_by_id {
    ($entity_type: path, $id: expr) => {
        $crate::resource::Resource::with_extra_base_object_pools_mut(|v, _| {
            match v.entity.get_by_id($id) {
                Some(_wrapper @ $entity_type(entity)) => Some(std::rc::Rc::clone(entity)),
                None | Some(_) => None,
            }
        })
    };
}

pub(crate) use __get_entity_by_id as get_entity_by_id;
