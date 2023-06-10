mod mvalue;
mod base_object_pool_funcs;
mod base_object_funcs;
mod helpers;
mod weapon_model_info;
mod player;

use base_object_funcs::test_base_object_funcs;
use base_object_pool_funcs::test_base_object_pool_funcs;
use mvalue::test_mvalue;
use player::test_player;
use weapon_model_info::test_weapon_model_info;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::log!("#################### test_base_object_funcs");
    test_base_object_funcs();
    altv::log!("#################### test_base_object_pool_funcs");
    test_base_object_pool_funcs();
    altv::log!("#################### mvalue");
    test_mvalue();
    altv::log!("#################### weapon_model_info");
    test_weapon_model_info();
    altv::log!("#################### player");
    test_player();

    altv::stop_server();
}
