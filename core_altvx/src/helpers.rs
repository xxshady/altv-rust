use crate::{
    base_object::BaseObject,
    base_object_maps::BaseObjectMap,
    player::{self, PlayerContainer},
    resource::Resource,
    vector::{Vector2, Vector3},
};
use altv_sdk::ffi as sdk;
use autocxx::prelude::*;

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
    get_target: unsafe fn(*const T) -> *mut sdk::alt::IPlayer,
) -> PlayerContainer {
    let raw_ptr = {
        let player_raw_ptr = unsafe { get_target(event) };

        if player_raw_ptr.is_null() {
            panic!("get_event_player_target returned null player ptr");
        }

        unsafe { sdk::player::to_base_object(player_raw_ptr) }
    };

    let players = resource.player_base_object_map.borrow();
    players.get_by_base_object_ptr(raw_ptr).unwrap()
}

pub fn get_player_raw_ptr(
    player: player::PlayerContainer,
) -> anyhow::Result<*mut sdk::alt::IPlayer> {
    player.try_borrow_mut()?.ptr_mut().to_player()
}

pub fn read_cpp_vector3(cpp_vector: UniquePtr<sdk::Vector3Wrapper>) -> Vector3 {
    let out_x = Box::into_raw(Box::new(0f32));
    let out_y = Box::into_raw(Box::new(0f32));
    let out_z = Box::into_raw(Box::new(0f32));
    unsafe {
        sdk::read_vector3(cpp_vector.as_ref().unwrap(), out_x, out_y, out_z);
        Vector3::new(*out_x, *out_y, *out_z)
    }
}

pub fn read_cpp_vector2(cpp_vector: UniquePtr<sdk::Vector2Wrapper>) -> Vector2 {
    let out_x = Box::into_raw(Box::new(0f32));
    let out_y = Box::into_raw(Box::new(0f32));
    unsafe {
        sdk::read_vector2(cpp_vector.as_ref().unwrap(), out_x, out_y);
        Vector2::new(*out_x, *out_y)
    }
}
