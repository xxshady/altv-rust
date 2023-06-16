pub(crate) fn test_weapon_model_info() {
    let none = altv::WeaponModelInfo::get_by_hash("awdadwdwad");
    assert!(none.is_none());

    let pistol = altv::WeaponModelInfo::get_by_hash("weapon_pistol").unwrap();
    assert_eq!(pistol.name, "WEAPON_PISTOL");
    assert_eq!(pistol.ammo_type, "AMMO_PISTOL");
    assert_eq!(pistol.hash, altv::hash("weapon_pistol"));
    assert_eq!(pistol.ammo_type_hash, altv::AmmoType::Pistol as u32);
    assert_eq!(pistol.ammo_type_hash, altv::hash("ammo_pistol"));

    let unarmed = altv::WeaponModelInfo::get_by_hash("weapon_unarmed");
    assert_eq!(unarmed.unwrap().name, "WEAPON_UNARMED");

    let rpg = altv::WeaponModelInfo::get_by_hash("weapon_rpg");
    assert_eq!(rpg.unwrap().name, "WEAPON_RPG");
}
