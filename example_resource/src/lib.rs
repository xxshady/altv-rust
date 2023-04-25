pub use altv::prelude::*;

#[altv::main(crate_name = "altv")]
fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    // Sending local "example" event with one dict as argument
    altv::events::emit!("example", altv::mvalue::dict! { "example" => 123 }.unwrap()).unwrap();

    // Applying a per-resource vehicle meta
    let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();
    vehicle
        .set_meta("example", altv::mvalue::dict! { "example" => 123 }.unwrap())
        .unwrap();
}
