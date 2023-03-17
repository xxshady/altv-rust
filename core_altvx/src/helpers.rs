use altv_sdk::ffi::alt::IPlayer;

use crate::{
    base_object::BaseObject,
    player::{self, PlayerContainer},
    resource::Resource,
};

#[macro_export]
macro_rules! get_entity_by_id {
    ($wrapper: path, $entity_id: ident) => {
        $crate::resource::Resource::with_entities_ref(|entities, _| {
            let result = entities.get_by_id($entity_id);

            match result {
                Some(_wrapper @ $wrapper(entity)) => Some(Rc::clone(entity)),
                None | Some(_) => None,
            }
        })
    };
}

pub fn get_player_from_event<T>(
    event: *const T,
    resource: &Resource,
    get_target: unsafe fn(*const T) -> *mut IPlayer,
) -> PlayerContainer {
    let raw_ptr = {
        let player_raw_ptr = unsafe { get_target(event) };

        if player_raw_ptr.is_null() {
            panic!("get_event_player_target returned null player ptr");
        }

        unsafe { altv_sdk::ffi::player::to_base_object(player_raw_ptr) }
    };

    let players = resource.player_base_object_map.borrow();
    players.get_by_base_object_ptr(raw_ptr).unwrap()
}

pub fn get_player_raw_ptr(
    player: player::PlayerContainer,
) -> anyhow::Result<*mut altv_sdk::ffi::alt::IPlayer> {
    player.try_borrow_mut()?.ptr_mut().to_player()
}