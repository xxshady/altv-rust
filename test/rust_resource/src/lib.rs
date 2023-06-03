mod mvalue;
mod base_object_pool_funcs;
mod base_object_funcs;
mod helpers;

use base_object_funcs::test_base_object_funcs;
use base_object_pool_funcs::test_base_object_pool_funcs;
use mvalue::test_mvalue;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::log!("#################### test_base_object_funcs");
    test_base_object_funcs();
    altv::log!("#################### test_base_object_pool_funcs");
    test_base_object_pool_funcs();
    altv::log!("#################### mvalue");
    test_mvalue();

    altv::stop_server();
}
