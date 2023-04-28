pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let group = altv::VirtualEntityGroup::new(10);
    let ve = altv::VirtualEntity::new(group, (1, 2, 3), 10).unwrap();
    dbg!(ve.pos());

    let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();
    dbg!(vehicle.pos());

    let vehicle2 = altv::Vehicle::new("sultan", 0, 0).unwrap();

    dbg!(ve.id());
    dbg!(vehicle.id());
    dbg!(vehicle2.id());

    let blip = altv::Blip::new_point(0);
    dbg!(blip.pos());

    let colp = altv::ColShape::new_circle(0, 10.0);
    dbg!(colp.pos());

    dbg!(colp.is_point_in(9));

    let checkp = altv::Checkpoint::new(0, 0, 10.0, 10.0, (255, 255, 255), 10);

    dbg!(checkp.is_point_in(0).unwrap());
    dbg!(checkp.pos().unwrap());
    dbg!(checkp.checkpoint_type().unwrap());
    dbg!(checkp.col_shape_type().unwrap());
}
