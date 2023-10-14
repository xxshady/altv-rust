use altv::{Entity, prelude::*, meta::StreamSyncedEntityMetaEntry};

pub(crate) fn test_ped() {
    let ped = altv::Ped::new("mp_m_freemode_01", 0, 0).unwrap();
    assert_eq!(dbg!(ped.streaming_distance()).unwrap(), 300);

    let ped = altv::Ped::new_with_streaming_distance("mp_m_freemode_01", 0, 0, 10).unwrap();
    assert_eq!(dbg!(ped.streaming_distance()).unwrap(), 10);

    let entry: StreamSyncedEntityMetaEntry<i32> = ped.stream_synced_meta_entry("test").unwrap();

    assert!(!dbg!(entry.has().unwrap()));
    assert!(dbg!(entry.get().unwrap()).is_none());
    entry.set(&123).unwrap();
    assert_eq!(dbg!(entry.get().unwrap()), Some(123));
    assert!(dbg!(entry.has().unwrap()));
}
