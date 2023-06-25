use altv::{Blip, BlipType};

pub(crate) fn test_blip() {
    let area_blip = Blip::new_area(0.0, 1.0, 2.0);
    assert_blip_type(area_blip.blip_type().unwrap(), BlipType::Area);

    let scale = area_blip.scale().unwrap();
    dbg!(&scale);
    assert_eq!(scale.x(), 1.0);
    assert_eq!(scale.y(), 2.0);

    area_blip.destroy().unwrap();

    let point_blip = Blip::new_point(0.0);
    assert_blip_type(point_blip.blip_type().unwrap(), BlipType::Destination);
    point_blip.destroy().unwrap();

    let radius_blip = Blip::new_radius(0.0, 10.0);
    assert_blip_type(radius_blip.blip_type().unwrap(), BlipType::Radius);

    let scale = radius_blip.scale().unwrap();
    dbg!(&scale);
    assert_eq!(scale.x(), 10.0);
    assert_eq!(scale.y(), 10.0);

    radius_blip.destroy().unwrap();
}

fn assert_blip_type(blip_type: BlipType, expected: BlipType) {
    dbg!(blip_type);
    assert_eq!(blip_type, expected);
}
