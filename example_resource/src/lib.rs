pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let group = altv::VirtualEntityGroup::new(10);
    let ent = altv::VirtualEntity::new(group, (1, 2, 3), 10).unwrap();
    dbg!(ent.pos());
    ent.destroy().unwrap();
    dbg!(ent.pos());
}
