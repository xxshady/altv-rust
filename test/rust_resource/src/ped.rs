use altv::Entity;

pub(crate) fn test_ped() {
    let ped = altv::Ped::new("mp_m_freemode_01", 0, 0).unwrap();
    assert_eq!(dbg!(ped.streaming_distance()).unwrap(), 300);

    let ped = altv::Ped::new_with_streaming_distance("mp_m_freemode_01", 0, 0, 10).unwrap();
    assert_eq!(dbg!(ped.streaming_distance()).unwrap(), 10);
}
