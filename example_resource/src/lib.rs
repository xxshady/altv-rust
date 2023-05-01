pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();

    let rgba = altv::RGBA::new(1, 2, 3, 4);
    vehicle.set_meta("example", &rgba).unwrap();
    vehicle.set_stream_synced_meta("test", 123).unwrap();
    vehicle.get_stream_synced_meta("test").unwrap();
    vehicle.has_stream_synced_meta("test").unwrap();
    vehicle.delete_stream_synced_meta("test").unwrap();
    vehicle.get_stream_synced_meta_keys().unwrap();

    dbg!(vehicle.get_meta("example").unwrap());

    let g = altv::VirtualEntityGroup::new(10);
    let ve = altv::VirtualEntity::new(g, 0, 0).unwrap();
    ve.get_stream_synced_meta("test").unwrap();
    ve.get_synced_meta("kek").unwrap();
    ve.get_meta("test").unwrap();

    let ch = altv::Checkpoint::new(0, 0, 0.0, 0.0, (255, 255, 255), 0);

    ch.get_stream_synced_meta("test").unwrap();
    ch.get_synced_meta("kek").unwrap();
    ch.get_meta("test").unwrap();

    dbg!(rgba);

    altv::set_timeout(
        move || {
            dbg!(vehicle.get_meta("example").unwrap());
        },
        1000,
    );

    dbg!(altv::set_meta("kek", 123).unwrap());
    dbg!(altv::get_meta("kek"));
    dbg!(altv::has_meta("kek"));
    dbg!(altv::delete_meta("kek"));
    dbg!(altv::get_meta("kek"));
    dbg!(altv::has_meta("kek"));

    dbg!(altv::set_synced_meta("kek", 123).unwrap());
    dbg!(altv::get_synced_meta("kek"));
    dbg!(altv::has_synced_meta("kek"));
    dbg!(altv::delete_synced_meta("kek"));
    dbg!(altv::get_synced_meta("kek"));
    dbg!(altv::has_synced_meta("kek"));
}
