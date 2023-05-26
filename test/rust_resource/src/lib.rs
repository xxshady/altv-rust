mod mvalue;

use mvalue::test_mvalue;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "0");

    test_mvalue();

    altv::stop_server();
}
