use crate::{
    base_object::player,
    helpers::get_player_raw_ptr,
    mvalue::{convert_player_vec_to_cpp_vec, convert_vec_to_mvalue_vec, Serializable},
    VoidResult,
};
use altv_sdk::ffi as sdk;

pub fn emit_all_clients(event_name: &str, args: Vec<Serializable>) {
    unsafe {
        sdk::trigger_client_event_for_all(event_name, convert_vec_to_mvalue_vec(args));
    }
}

pub fn emit_all_clients_without_args(event_name: &str) {
    unsafe {
        sdk::trigger_client_event_for_all(event_name, sdk::create_mvalue_vec());
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
            convert_vec_to_mvalue_vec(args),
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

pub fn emit_client(
    event_name: &str,
    player: player::PlayerContainer,
    args: Vec<Serializable>,
) -> VoidResult {
    unsafe {
        sdk::trigger_client_event(
            get_player_raw_ptr(player)?,
            event_name,
            convert_vec_to_mvalue_vec(args),
        );
    }
    Ok(())
}

pub fn emit_client_without_args(event_name: &str, player: player::PlayerContainer) -> VoidResult {
    unsafe {
        sdk::trigger_client_event(
            get_player_raw_ptr(player)?,
            event_name,
            sdk::create_mvalue_vec(),
        );
    }
    Ok(())
}

#[macro_export]
macro_rules! __emit_client {
    ($event_name: expr, $player: expr) => {
        $crate::client_events::emit_client_without_args($event_name, $player)
    };
    ($event_name: expr, $player: expr, $( $arg: expr ),+ ) => {
        (|| -> $crate::VoidResult {
            let vec = $crate::mvalue_list!($( $arg ),+)?;
            $crate::client_events::emit_client(
                $event_name,
                $player,
                vec
            )
        })()
    };
}

#[macro_export]
macro_rules! __emit_all_clients {
    ($event_name: expr) => {
        $crate::client_events::emit_all_clients($event_name)
    };
    ($event_name: expr, $( $arg: expr ),+ ) => {
        (|| -> $crate::VoidResult {
            let vec = $crate::mvalue_list!($( $arg ),+)?;
            $crate::client_events::emit_all_clients(
                $event_name,
                vec
            );
            Ok(())
        })()
    };
}

#[macro_export]
macro_rules! __emit_some_clients {
    ($event_name: expr, $players: expr) => {
        $crate::client_events::emit_some_clients_without_args($event_name, $players)
    };
    ($event_name: expr, $players: expr, $( $arg: expr ),+ ) => {
        (|| -> $crate::VoidResult {
            let vec = $crate::mvalue_list!($( $arg ),+)?;
            $crate::client_events::emit_some_clients(
                $event_name,
                $players,
                vec
            )
        })()
    };
}
