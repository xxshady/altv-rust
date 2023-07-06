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

mod script_events;
use script_events::test_script_events;

mod blip;
use blip::test_blip;

mod core_funcs;
use core_funcs::test_core_funcs;

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
    altv::log!("#################### script_events");
    test_script_events();
    altv::log!("#################### blip");
    test_blip();
    altv::log!("#################### core_funcs");
    test_core_funcs();

    altv::set_timeout(
        || {
            altv::log!("stopping resource...");
            altv::Resource::current().stop();
        },
        1000,
    );
}
