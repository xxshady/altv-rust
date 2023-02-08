use crate::{
    base_object::{BaseObject, BaseObjectPointer},
    entity::{Entity, EntityId},
};
use once_cell::sync::OnceCell;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub(crate) static PLAYER_MANAGER_INSTANCE: OnceCell<Mutex<PlayerManager>> = OnceCell::new();

pub type PlayerContainer = Arc<Mutex<Player>>;

#[derive(Debug)]
pub struct Player {
    ptr: BaseObjectPointer,
    base_type: altv_sdk::BaseObjectType,
}

impl Player {
    pub fn get_by_id(id: EntityId) -> Option<PlayerContainer> {
        PLAYER_MANAGER_INSTANCE
            .get()
            .unwrap()
            .try_lock()
            .unwrap()
            .players
            .get(&id)
            .cloned()
    }
}

impl BaseObject for Player {
    fn ptr(&self) -> &BaseObjectPointer {
        &self.ptr
    }

    fn ptr_mut(&mut self) -> &mut BaseObjectPointer {
        &mut self.ptr
    }

    fn base_type(&self) -> altv_sdk::BaseObjectType {
        self.base_type
    }
}
impl Entity for Player {}

#[derive(Debug)]
pub(crate) struct PlayerManager {
    players: HashMap<EntityId, PlayerContainer>,
}

impl PlayerManager {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }
}
