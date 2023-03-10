use std::cell::Ref;

use crate::{
    base_object::BaseObject,
    base_object_maps::PlayerBaseObjectMap,
    player::{self, PlayerContainer},
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

pub fn get_player_from_event(
    event: *const altv_sdk::ffi::alt::CEvent,
    players: &Ref<PlayerBaseObjectMap>,
) -> PlayerContainer {
    let raw_ptr = {
        let player_raw_ptr = unsafe { altv_sdk::ffi::get_event_player_target(event) };

        if player_raw_ptr.is_null() {
            panic!("get_event_player_target returned null player ptr");
        }

        unsafe { altv_sdk::ffi::convert_player_to_base_object(player_raw_ptr) }
    };

    players.get_by_base_object_ptr(raw_ptr).unwrap()
}

pub fn get_player_raw_ptr(
    player: player::PlayerContainer,
) -> anyhow::Result<*mut altv_sdk::ffi::alt::IPlayer> {
    player.try_borrow_mut()?.ptr_mut().to_player()
}
