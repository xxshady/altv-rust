use crate::{
    base_object::{BaseObject, BaseObjectPointer, RawBaseObjectPointer},
    entity::{Entity, EntityId, EntityWrapper},
    impl_base_object_for,
};
use altv_sdk::ffi as sdk;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type PlayerContainer = Rc<RefCell<Player>>;

#[derive(Debug)]
pub struct Player {
    ptr: BaseObjectPointer,
    base_type: altv_sdk::BaseObjectType,
}

impl Player {
    pub fn get_by_id(id: EntityId) -> Option<PlayerContainer> {
        crate::get_entity_by_id!(EntityWrapper::Player, id)
    }

    pub fn name(&self) -> Result<String, String> {
        Ok(unsafe { sdk::get_player_name(self.ptr.to_player()?) }.to_string())
    }
}

impl_base_object_for!(Player);
impl Entity for Player {}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn create_player_container(raw_ptr: RawBaseObjectPointer) -> PlayerContainer {
    Rc::new(RefCell::new(Player {
        ptr: BaseObjectPointer::new(raw_ptr),
        base_type: altv_sdk::BaseObjectType::VEHICLE,
    }))
}

#[derive(Debug)]
pub(crate) struct PlayerManager {
    players: HashMap<RawBaseObjectPointer, PlayerContainer>,
}

impl PlayerManager {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, player: PlayerContainer) {
        self.players
            .insert(player.borrow().ptr().get().unwrap(), Rc::clone(&player));
    }

    pub fn remove_player(&mut self, raw_ptr: RawBaseObjectPointer) {
        self.players.remove(&raw_ptr);
    }

    pub fn get_by_base_object_ptr(&self, raw_ptr: RawBaseObjectPointer) -> Option<PlayerContainer> {
        self.players.get(&raw_ptr).cloned()
    }
}
