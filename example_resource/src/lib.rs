pub use altv::prelude::*;

#[altv::main(crate_name = "altv")]
fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::log!("ok");
}
