use altv::{AmmoType, VoiceConnectionState};

macro_rules! test_property {
    ($name:path, $value:expr) => {
        paste::paste! {
            dbg!(altv::[<$name>]());
            altv::[<set_$name>]($value);
            assert_eq!(dbg!(altv::[<$name>]()), $value);
        }
    };
}

pub(crate) fn test_core_funcs() {
    let voice_state = altv::voice_connection_state();
    dbg!(voice_state);
    assert_eq!(voice_state, VoiceConnectionState::Disconnected);

    assert!(altv::get_ammo_hash_for_weapon_hash("weapon_pistolwdwawda").is_none());

    let pistol_ammo_hash = altv::get_ammo_hash_for_weapon_hash("weapon_pistol").unwrap();
    dbg!(pistol_ammo_hash);

    assert_eq!(AmmoType::Pistol as u32, pistol_ammo_hash);

    dbg!(altv::net_time());

    test_property!(streaming_distance, 50);
    test_property!(migration_distance, 50);
    test_property!(col_shape_tick_rate, 1000);
    test_property!(sync_send_thread_count, 1);

    let models = altv::loaded_vehicle_models();
    dbg!(&models.len());

    assert!(models.contains(&altv::hash("sultan")));
    assert!(models.contains(&altv::hash("sultan2")));
    assert!(models.contains(&altv::hash("sultan3")));
    assert!(models.contains(&altv::hash("elegy")));
}
