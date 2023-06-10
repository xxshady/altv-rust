use altv::BaseObjectPoolFuncs;

pub(crate) fn test_player() {
    // here we can only test correctness of declarations & types,
    // since player base object requires client to be connected
    fn _do_not_execute_this() {
        let player = altv::Player::get_by_id(123).unwrap();

        let _ammo: u16 = player.get_ammo(123).unwrap();
        let _ammo: u16 = player.get_weapon_ammo(123).unwrap();
    }
}
