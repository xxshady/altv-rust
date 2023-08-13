use crate::{base_objects::player, helpers::convert_player_slice_to_cpp_vec, VoidResult};
use altv_sdk::ffi as sdk;
use mvalue::serialize_args;

pub fn emit_all_players(event_name: impl ToString, args: mvalue::DynMValueArgs) -> VoidResult {
    unsafe {
        sdk::trigger_client_event_for_all(event_name.to_string(), serialize_args(args)?);
    }
    Ok(())
}

pub fn emit_all_players_unreliable(
    event_name: impl ToString,
    args: mvalue::DynMValueArgs,
) -> VoidResult {
    unsafe {
        sdk::trigger_client_event_unreliable_for_all(event_name.to_string(), serialize_args(args)?);
    }
    Ok(())
}

pub fn emit_some_players(
    event_name: impl ToString,
    players: &[player::PlayerContainer],
    args: mvalue::DynMValueArgs,
) -> VoidResult {
    unsafe {
        sdk::trigger_client_event_for_some(
            convert_player_slice_to_cpp_vec(players)?,
            event_name.to_string(),
            serialize_args(args)?,
        );
    }
    Ok(())
}

pub fn emit_some_players_unreliable(
    event_name: impl ToString,
    players: &[player::PlayerContainer],
    args: mvalue::DynMValueArgs,
) -> VoidResult {
    unsafe {
        sdk::trigger_client_event_unreliable_for_some(
            convert_player_slice_to_cpp_vec(players)?,
            event_name.to_string(),
            serialize_args(args)?,
        );
    }
    Ok(())
}
