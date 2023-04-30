pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();

    let rgba = altv::RGBA::new(1, 2, 3, 4);
    vehicle.set_meta("example", &rgba).unwrap();
    dbg!(vehicle.get_meta("example").unwrap());

    dbg!(rgba);

    altv::set_timeout(
        move || {
            dbg!(vehicle.get_meta("example").unwrap());
        },
        1000,
    );
}
