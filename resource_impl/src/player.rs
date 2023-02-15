use crate::{
    base_object::{BaseObject, BaseObjectPointer, RawBaseObjectPointer},
    entity::{Entity, EntityId, EntityWrapper},
    resource_impl::RESOURCE_IMPL_INSTANCE,
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
    // TODO: test this
    pub fn get_by_id(id: EntityId) -> Option<PlayerContainer> {
        RESOURCE_IMPL_INSTANCE.with(|instance| {
            let instance = instance.borrow();
            let entities = instance.borrow_entities();
            let result = entities.get_by_id(id);

            match result {
                Some(_wrapper @ EntityWrapper::Player(player)) => Some(Rc::clone(player)),
                None | Some(_) => None,
            }
        })
    }

    pub fn name(&self) -> Result<String, String> {
        Ok(unsafe { sdk::get_player_name(self.ptr.to_player()?) }.to_string())
    }
}

impl BaseObject for Player {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

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

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn create_player_container(raw_ptr: RawBaseObjectPointer) -> PlayerContainer {
    Rc::new(RefCell::new(Player {
        ptr: BaseObjectPointer::new(raw_ptr),
        base_type: altv_sdk::BaseObjectType::VEHICLE,
    }))
}

#[derive(Debug)]
pub(crate) struct PlayerManager {
    // usize is RawBaseObjectPointer
    players: HashMap<usize, PlayerContainer>,
}

impl PlayerManager {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, player: PlayerContainer) {
        self.players.insert(
            player.borrow().ptr().get().unwrap() as usize,
            Rc::clone(&player),
        );
    }

    pub fn remove_player(&mut self, raw_ptr: RawBaseObjectPointer) {
        self.players.remove(&(raw_ptr as usize));
    }

    pub fn get_by_base_object_ptr(&self, raw_ptr: RawBaseObjectPointer) -> Option<PlayerContainer> {
        self.players.get(&(raw_ptr as usize)).cloned()
    }
}
