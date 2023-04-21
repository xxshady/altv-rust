pub use altv::prelude::*;

#[altv::main(crate_name = "altv")]
fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    dbg!(altv::VehicleModelInfo::get_by_hash("sultan2"));
}
