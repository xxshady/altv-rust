use altv::meta::{BaseObjectMetaEntry, NormalBaseObjectMeta, StreamSyncedEntityMeta};
use altv::{
    events::{on_global_meta_change, on_stream_synced_meta_change},
    Vehicle,
};

pub fn test_metadata_events() {
    on_global_meta_change(|ctx| {
        altv::log!("global meta change {ctx:?}");
    });

    on_stream_synced_meta_change(|ctx| {
        altv::log!("stream synced meta change {ctx:?}");
    });

    let veh = Vehicle::new("sultan3", 0, 0).unwrap();

    // wont call global meta change (current alt:V core behavior)
    veh.meta_entry("test").unwrap().set(&true).unwrap();

    veh.stream_synced_meta_entry("test")
        .unwrap()
        .set(&true)
        .unwrap();
}
