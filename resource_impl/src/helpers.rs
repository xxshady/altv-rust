use std::cell::Ref;

use crate::{base_object_maps::PlayerBaseObjectMap, player::PlayerContainer};

#[macro_export]
macro_rules! get_entity_by_id {
    ($wrapper: path, $entity_id: ident) => {
        $crate::resource_impl::with_resource_impl(|instance| {
            let entities = instance.entities.borrow();
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
