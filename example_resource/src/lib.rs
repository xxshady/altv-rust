pub use altv::prelude::*;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    let colshape = altv::ColShape::new_circle(0, 10.0);
    let vehicle = altv::Vehicle::new("sultan2", 0, 0)?;
    colshape.is_entity_in(vehicle).unwrap();

    let base_object = altv::AnyBaseObject::ColShape(colshape);
    let entity: altv::AnyEntity = base_object.try_into()?;

    Ok(())
}
