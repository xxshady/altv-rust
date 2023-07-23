use altv::{AmmoType, VoiceConnectionState};

pub(crate) fn test_core_funcs() {
    let voice_state = altv::get_voice_connection_state();
    dbg!(voice_state);
    assert_eq!(voice_state, VoiceConnectionState::Disconnected);

    assert!(altv::get_ammo_hash_for_weapon_hash("weapon_pistolwdwawda").is_none());

    let pistol_ammo_hash = altv::get_ammo_hash_for_weapon_hash("weapon_pistol").unwrap();
    dbg!(pistol_ammo_hash);

    assert_eq!(AmmoType::Pistol as u32, pistol_ammo_hash);
}
