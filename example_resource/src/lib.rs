pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();

    let bool: Option<bool> = Some(true);
    vehicle.set_meta("example", bool).unwrap();
    // outputs "Bool(true)", because MValue does not know anything about Rust's Option
    dbg!(vehicle.get_meta("example").unwrap());

    let none: Option<bool> = None;
    vehicle.set_meta("example", none).unwrap();
    // outputs "None"
    dbg!(vehicle.get_meta("example").unwrap());
}
