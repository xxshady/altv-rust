pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();

    let buf: Vec<u8> = vec![1, 2, 3];
    dbg!(buf.len(), buf.capacity());

    vehicle.set_meta("example", buf.as_slice()).unwrap();
    dbg!(vehicle.get_meta("example").unwrap());

    altv::set_timeout(
        move || {
            dbg!(vehicle.get_meta("example").unwrap());
        },
        1000,
    );
}
