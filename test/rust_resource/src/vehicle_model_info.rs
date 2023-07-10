pub(crate) fn test_vehicle_model_info() {
    let none = altv::VehicleModelInfo::get_by_hash("awdadwdwad");
    assert!(none.is_none());

    let sultan = altv::VehicleModelInfo::get_by_hash("sultan").unwrap();
    dbg!(&sultan.title);
    assert_eq!(sultan.title, "sultan");
    assert_eq!(sultan.model_type, altv::VehicleModelType::Automobile);
}
