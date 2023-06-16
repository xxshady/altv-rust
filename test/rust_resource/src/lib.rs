mod helpers;

mod mvalue;
use mvalue::test_mvalue;

mod base_object_pool_funcs;
use base_object_pool_funcs::test_base_object_pool_funcs;

mod base_object_funcs;
use base_object_funcs::test_base_object_funcs;

mod weapon_model_info;
use weapon_model_info::test_weapon_model_info;

mod timers;
use timers::test_timers;

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
    altv::log!("#################### timers");
    test_timers();

    altv::set_timeout(|| altv::stop_server(), 1000);
}
