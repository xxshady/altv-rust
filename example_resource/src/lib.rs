pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let c = altv::ColShape::new_circle(0, 10.0);
    dbg!(c.is_point_in(0).unwrap());
    dbg!(c.is_point_in(5000).unwrap());
}
