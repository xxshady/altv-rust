use crate::{
    base_objects::player,
    mvalue::{convert_iter_to_mvalue_vec, convert_player_vec_to_cpp_vec, Serializable},
    VoidResult,
};
use altv_sdk::ffi as sdk;

pub fn emit_all_clients(event_name: &str, args: Vec<Serializable>) {
    unsafe {
        sdk::trigger_client_event_for_all(event_name, convert_iter_to_mvalue_vec(args));
    }
}

pub fn emit_all_clients_unreliable(event_name: &str, args: Vec<Serializable>) {
    unsafe {
        sdk::trigger_client_event_unreliable_for_all(event_name, convert_iter_to_mvalue_vec(args));
    }
}

pub fn emit_all_clients_without_args(event_name: &str) {
    unsafe {
        sdk::trigger_client_event_for_all(event_name, sdk::create_mvalue_vec());
    }
}

pub fn emit_all_clients_unreliable_without_args(event_name: &str) {
    unsafe {
        sdk::trigger_client_event_unreliable_for_all(event_name, sdk::create_mvalue_vec());
    }
}

pub fn emit_some_clients(
    event_name: &str,
    players: Vec<player::PlayerContainer>,
    args: Vec<Serializable>,
) -> VoidResult {
    unsafe {
        sdk::trigger_client_event_for_some(
            convert_player_vec_to_cpp_vec(players)?,
            event_name,
            convert_iter_to_mvalue_vec(args),
        );
    }
    Ok(())
}

pub fn emit_some_clients_without_args(
    event_name: &str,
    players: Vec<player::PlayerContainer>,
) -> VoidResult {
    unsafe {
        sdk::trigger_client_event_for_some(
            convert_player_vec_to_cpp_vec(players)?,
            event_name,
            sdk::create_mvalue_vec(),
        );
    }
    Ok(())
}

pub fn emit_some_clients_unreliable(
    event_name: &str,
    players: Vec<player::PlayerContainer>,
    args: Vec<Serializable>,
) -> VoidResult {
    unsafe {
        sdk::trigger_client_event_unreliable_for_some(
            convert_player_vec_to_cpp_vec(players)?,
            event_name,
            convert_iter_to_mvalue_vec(args),
        );
    }
    Ok(())
}

pub fn emit_some_clients_unreliable_without_args(
    event_name: &str,
    players: Vec<player::PlayerContainer>,
) -> VoidResult {
    unsafe {
        sdk::trigger_client_event_unreliable_for_some(
            convert_player_vec_to_cpp_vec(players)?,
            event_name,
            sdk::create_mvalue_vec(),
        );
    }
    Ok(())
}

pub fn emit_client(
    event_name: &str,
    player: player::PlayerContainer,
    args: Vec<Serializable>,
) -> VoidResult {
    unsafe {
        sdk::trigger_client_event(
            player.raw_ptr()?,
            event_name,
            convert_iter_to_mvalue_vec(args),
        );
    }
    Ok(())
}

pub fn emit_client_without_args(event_name: &str, player: player::PlayerContainer) -> VoidResult {
    unsafe {
        sdk::trigger_client_event(player.raw_ptr()?, event_name, sdk::create_mvalue_vec());
    }
    Ok(())
}

pub fn emit_client_unreliable(
    event_name: &str,
    player: player::PlayerContainer,
    args: Vec<Serializable>,
) -> VoidResult {
    unsafe {
        sdk::trigger_client_event_unreliable(
            player.raw_ptr()?,
            event_name,
            convert_iter_to_mvalue_vec(args),
        );
    }
    Ok(())
}

pub fn emit_client_unreliable_without_args(
    event_name: &str,
    player: player::PlayerContainer,
) -> VoidResult {
    unsafe {
        sdk::trigger_client_event_unreliable(
            player.raw_ptr()?,
            event_name,
            sdk::create_mvalue_vec(),
        );
    }
    Ok(())
}
