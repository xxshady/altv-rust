mod mvalue;
mod base_object_pool_funcs;
mod base_object_funcs;
mod helpers;
mod weapon_model_info;

use base_object_funcs::test_base_object_funcs;
use base_object_pool_funcs::test_base_object_pool_funcs;
use mvalue::test_mvalue;
use weapon_model_info::test_weapon_model_info;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    // altv::log!("#################### test_base_object_funcs");
    // test_base_object_funcs();
    // altv::log!("#################### test_base_object_pool_funcs");
    // test_base_object_pool_funcs();
    // altv::log!("#################### mvalue");
    // test_mvalue();
    // altv::log!("#################### weapon_model_info");
    // test_weapon_model_info();

    // altv::stop_server();

    altv::events::on_player_connect(|c| {
        altv::log!("player connect: {}", c.player.name().unwrap());
        const WEAPON: &str = "weapon_pistol";

        let player = c.player.clone();

        player.spawn("mp_m_freemode_01", (0, 0, 72)).unwrap();
        player.give_weapon(WEAPON, 999, true).unwrap();

        altv::set_interval(
            move || {
                altv::log!(
                    "player get_weapon_ammo: {}",
                    player.get_weapon_ammo(WEAPON).unwrap()
                );
                let weapon_info = altv::WeaponModelInfo::get_by_hash(WEAPON).unwrap();
                altv::log!("player get_ammo: {}", player.get_ammo(weapon_info.ammo_type_hash).unwrap());
            },
            1500,
        );
    });
}
