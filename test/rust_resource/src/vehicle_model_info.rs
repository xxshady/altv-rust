pub(crate) fn test_vehicle_model_info() {
    let none = altv::VehicleModelInfo::get_by_hash("awdadwdwad");
    assert!(none.is_none());

    let sultan = altv::VehicleModelInfo::get_by_hash("sultan").unwrap();
    dbg!(&sultan.title);
    assert_eq!(sultan.title, "sultan");
    assert_eq!(sultan.model_type, altv::VehicleModelType::Automobile);
    assert_eq!(sultan.handling_name_hash, altv::hash("sultan"));

    let mesa3 = altv::VehicleModelInfo::get_by_hash("mesa3").unwrap();
    dbg!(&mesa3.title);
    assert_eq!(mesa3.title, "MESA3");
    assert_eq!(mesa3.model_type, altv::VehicleModelType::Automobile);
    assert_eq!(mesa3.handling_name_hash, altv::hash("mesa"));
}
