pub(crate) fn test_weapon_model_info() {
    let none = altv::WeaponModelInfo::get_by_hash("awdadwdwad");
    assert!(none.is_none());

    let pistol = altv::WeaponModelInfo::get_by_hash("weapon_pistol");
    assert_eq!(pistol.unwrap().name, "WEAPON_PISTOL");

    let unarmed = altv::WeaponModelInfo::get_by_hash("weapon_unarmed");
    assert_eq!(unarmed.unwrap().name, "WEAPON_UNARMED");

    let rpg = altv::WeaponModelInfo::get_by_hash("weapon_rpg");
    assert_eq!(rpg.unwrap().name, "WEAPON_RPG");
}
