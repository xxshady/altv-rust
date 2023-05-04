pub use altv::prelude::*;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    let colshape = altv::ColShape::new_circle(0, 10.0);
    let vehicle = altv::Vehicle::all()[0].clone();
    colshape.is_entity_in(vehicle).unwrap();
}
