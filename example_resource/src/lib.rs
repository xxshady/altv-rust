pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let veh = alt::Vehicle::new("s80", 0, alt::Vector3::new(0.0, 0, 0)).unwrap();
    veh.set_pos(0).unwrap();
    veh.set_pos((0, 0, 1.5)).unwrap();
    veh.set_quaternion((0, 0, 1.5, 0)).unwrap();
}
