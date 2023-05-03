pub use altv::prelude::*;

#[altv::main]
fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let test = || -> altv::anyhow::Result<_> {
        altv::meta::entry("normal global meta").set(123)?;
        altv::meta::entry("normal global meta 2").set(123)?;
        // altv::meta::synced_entry("synced global meta").set(123)?;
        // altv::meta::synced_entry("synced global meta 2").set(123)?;

        // dbg!(altv::meta::keys());
        // dbg!(altv::meta::synced_keys());

        // // altv::meta::synced_entry("synced global meta").delete();
        // altv::meta::synced_entry("synced global meta 2").delete();

        // dbg!(altv::meta::keys());
        // dbg!(altv::meta::synced_keys());

        // let ch = altv::Checkpoint::new(0, 0, 10.0, 10.0, (255, 255, 255), 10);

        // dbg!(ch.stream_synced_meta_keys()?);

        // ch.stream_synced_meta_entry("test")?.set(123)?;

        // dbg!(ch.stream_synced_meta_keys()?);

        // ch.meta_entry("meta_keys")?.set(true)?;

        // dbg!(ch.meta_keys()?);

        // ch.synced_meta_entry("synced_meta_keys")?.set(true)?;

        // let marker = altv::Marker::new(altv::MarkerType::Markerarrow, 0, (255, 255, 255));

        // marker.synced_meta_entry("synced_meta_keys")?.set(true)?;

        // dbg!(ch.synced_meta_keys()?);

        // dbg!(marker.synced_meta_keys()?);
        // dbg!(marker.meta_keys()?);

        // dbg!(marker.meta_keys()?);
        // dbg!(marker.meta_keys()?);
        // dbg!(marker.meta_keys()?);
        // dbg!(marker.meta_keys()?);
        // dbg!(marker.meta_keys()?);
        // dbg!(marker.meta_keys()?);

        // marker.destroy()?;

        // dbg!(marker.meta_keys()?);

        Ok(())
    };

    dbg!(test());
}
