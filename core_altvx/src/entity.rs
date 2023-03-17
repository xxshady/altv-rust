use std::{collections::HashMap, fmt::Debug};

use altv_sdk::ffi as sdk;

use crate::{player::PlayerContainer, vehicle::VehicleContainer, world_object::WorldObject};

pub type RawEntityPointer = *mut sdk::alt::IEntity;
pub type EntityId = u16;

pub trait Entity: WorldObject {
    fn id(&self) -> anyhow::Result<EntityId> {
        Ok(unsafe { sdk::IEntity::GetID(self.ptr().to_entity()?) })
    }

    fn model(&self) -> anyhow::Result<u32> {
        Ok(unsafe { sdk::IEntity::GetModel(self.ptr().to_entity()?) })
    }
}

impl Debug for dyn Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dyn Entity <any>")
    }
}

#[derive(Debug)]
pub enum EntityWrapper {
    Vehicle(VehicleContainer),
    Player(PlayerContainer),
}

#[derive(Debug, Default)]
pub struct EntityManager {
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
        let id = unsafe { sdk::IEntity::GetID(raw_ptr) };
        self.entities.remove(&id)
    }
}
