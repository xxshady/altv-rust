use crate::{
    base_object::{BaseObject, BaseObjectPointer},
    entity::{Entity, EntityId, EntityWrapper},
    impl_base_object_for,
};
use altv_sdk::ffi as sdk;
use std::{cell::RefCell, rc::Rc};

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

    pub fn name(&self) -> anyhow::Result<String> {
        Ok(unsafe { sdk::get_player_name(self.ptr.to_player()?) }.to_string())
    }
}

impl_base_object_for!(Player);
impl Entity for Player {}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn create_player_container(raw_ptr: altv_sdk::IBaseObjectMutPtr) -> PlayerContainer {
    Rc::new(RefCell::new(Player {
        ptr: BaseObjectPointer::new(raw_ptr),
        base_type: altv_sdk::BaseObjectType::VEHICLE,
    }))
}
