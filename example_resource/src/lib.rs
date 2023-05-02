pub use altv::prelude::*;

#[altv::main]
fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let test = || -> altv::anyhow::Result<_> {
        let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();
        vehicle
            .stream_synced_meta_entry("example")
            .unwrap()
            .set(123)
            .unwrap();

        vehicle
            .stream_synced_meta_entry("example")
            .unwrap()
            .get()
            .unwrap(); // MValue::I64(123)

        vehicle.stream_synced_meta_entry("example")?.get()?; // MValue::I64(123)
        vehicle.meta_entry("example")?.get()?;
        vehicle.synced_meta_entry("example")?.get()?;

        altv::meta::entry("example").set(123)?;

        let _value = altv::meta::entry("example").get();

        altv::meta::synced_entry("example").set(123)?;
        let _value = altv::meta::synced_entry("example").get();

        let ch = altv::Checkpoint::new(0, 0, 10.0, 10.0, (255, 255, 255), 10);

        let _value = ch.stream_synced_meta_entry("example")?.get()?;
        let _value = ch.meta_entry("example")?.get()?;
        let _value = ch.synced_meta_entry("example")?.get()?;

        let all = altv::Player::all();
        let player = all.get(0);

        if let Some(player) = player {
            // Set "example" key of stream synced meta to `123`
            player.local_meta_entry("example")?.set(123)?;

            // Read "example" key of stream synced meta
            player.local_meta_entry("example")?.get()?; // Some(MValue::I64(123))

            player.stream_synced_meta_entry("example")?.has()?;
        }

        let group = altv::VirtualEntityGroup::new(10);
        let entity = altv::VirtualEntity::new(group, altv::Vector3::new(0, 0, 72), 10)?;
        // Set "example" key of stream synced meta to `123`
        entity.stream_synced_meta_entry("example")?.set(123)?;
        // Read "example" key of stream synced meta
        let _value = entity.stream_synced_meta_entry("example")?.get()?; // Some(MValue::I64(123))

        let _value = entity.meta_entry("example")?.has()?;

        let object = altv::NetworkObject::new("model", 0, 0)?;

        dbg!(object.meta_entry("example")?.has()?);
        dbg!(object.synced_meta_entry("example")?.has()?);
        dbg!(object.stream_synced_meta_entry("example")?.has()?);

        Ok(())
    };
    test().unwrap();
}
