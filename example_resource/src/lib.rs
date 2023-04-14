pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let vehicle = alt::Vehicle::new("s80", 0, 0).unwrap();
    alt::set_timeout(
        move || {
            alt::log!("vehicle dimension: {}", vehicle.dimension().unwrap());
            dbg!(alt::Vehicle::all());
            let types = 2;
            dbg!(alt::get_closest_entities(0, 1000, 0, 1000, types));
            dbg!(alt::get_entities_in_dimension(0, types));
            dbg!(alt::get_entities_in_range(0, 1000, 0, types));
            Ok(())
        },
        300,
    );
}
