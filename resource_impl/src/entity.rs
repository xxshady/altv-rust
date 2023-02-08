use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};

use altv_sdk::ffi as sdk;
use once_cell::sync::OnceCell;

use crate::{base_object::BaseObject, player::PlayerContainer, vehicle::VehicleContainer};

pub type RawEntityPointer = *mut sdk::IEntity;
pub type EntityId = u16;

pub(crate) static ENTITY_MANAGER_INSTANCE: OnceCell<Mutex<EntityManager>> = OnceCell::new();

pub trait Entity: BaseObject {
    fn id(&self) -> Result<EntityId, String> {
        Ok(unsafe { sdk::get_entity_id(self.ptr().to_entity()?) })
    }
}

pub(crate) type EntityContainer = Arc<Mutex<dyn Entity + Send + Sync>>;

impl Debug for dyn Entity + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dyn Entity <any>")
    }
}

#[derive(Debug)]
pub(crate) enum EntityWrapper {
    Vehicle(VehicleContainer),
    Player(PlayerContainer),
}

#[derive(Debug, Default)]
pub(crate) struct EntityManager {
    entities: HashMap<EntityId, EntityWrapper>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn get_by_id(&self, id: EntityId) -> Option<&EntityWrapper> {
        self.entities.get(&id)
    }

    pub fn on_create(&mut self, id: EntityId, entity: EntityWrapper) {
        self.entities.insert(id, entity);
    }

    pub fn on_destroy(&mut self, raw_ptr: RawEntityPointer) -> Option<EntityWrapper> {
        let id = unsafe { sdk::get_entity_id(raw_ptr) };
        self.entities.remove(&id)
    }
}
