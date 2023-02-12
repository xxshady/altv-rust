use crate::{
    base_object::{BaseObject, BaseObjectPointer, RawBaseObjectPointer},
    entity::{Entity, EntityId, EntityWrapper, ENTITY_MANAGER_INSTANCE},
};
use altv_sdk::ffi as sdk;
use std::sync::{Arc, Mutex};

pub type PlayerContainer = Arc<Mutex<Player>>;

#[derive(Debug)]
pub struct Player {
    ptr: BaseObjectPointer,
    base_type: altv_sdk::BaseObjectType,
}

impl Player {
    pub fn get_by_id(id: EntityId) -> Option<PlayerContainer> {
        let manager = ENTITY_MANAGER_INSTANCE.get().unwrap().try_lock().unwrap();
        let result = manager.get_by_id(id);

        dbg!(result);

        match result {
            Some(_wrapper @ EntityWrapper::Player(player)) => Some(player.clone()),
            None | Some(_) => None,
        }
    }

    pub fn name(&self) -> Result<String, String> {
        Ok(unsafe { sdk::get_player_name(self.ptr.to_player()?) }.to_string())
    }
}

impl BaseObject for Player {
    fn as_any(&self) -> &dyn std::any::Any {
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
    Arc::new(Mutex::new(Player {
        ptr: BaseObjectPointer::new(raw_ptr),
        base_type: altv_sdk::BaseObjectType::VEHICLE,
    }))
}
