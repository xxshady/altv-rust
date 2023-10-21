use std::collections::HashMap;

use altv::{Entity, prelude::*, meta::StreamSyncedEntityMetaEntry, MValueHashMap, mvalue::DynMValue};

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

    let meta = HashMap::from([
        ("a".to_string(), &123),
        ("b".to_string(), &1234),
        ("c".to_string(), &12345),
    ]);

    ped.set_multiple_stream_synced_meta(MValueHashMap::new(
        meta.clone()
            .into_iter()
            .map(|(key, value)| (key, value as DynMValue))
            .collect(),
    ))
    .unwrap();

    let keys = ped.stream_synced_meta_keys().unwrap();
    dbg!(&keys);

    meta.into_iter().for_each(|(key, expected)| {
        assert!(keys.contains(&key));

        let value = ped
            .stream_synced_meta_entry::<i32>(key)
            .unwrap()
            .get()
            .unwrap()
            .unwrap();

        assert_eq!(*expected, value);
    });
}
