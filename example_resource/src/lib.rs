use std::collections::HashMap;

pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let group = altv::VirtualEntityGroup::new(10);
    let entity = altv::VirtualEntity::new_with_stream_meta(
        group,
        altv::Vector3::new(0, 0, 72),
        10,
        HashMap::from([("example", 123)]),
    )
    .unwrap();
    dbg!(&entity);
}
