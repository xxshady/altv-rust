use altv::{prelude::*, Vector3};

pub(crate) fn test_col_shape() {
    let c = altv::ColShapeCircle::new(3, 10.0);
    let pos = c.pos().unwrap();
    assert_eq!(pos, Vector3::new(3, 3, 0));
    assert_eq!(c.col_shape_type().unwrap(), altv::ColShapeType::Circle);
    assert_eq!(c.radius().unwrap(), 10.0);

    let is_in = c.is_point_in(Vector3::new(0, 0, 999)).unwrap();
    assert!(is_in);

    let is_in = c.is_point_in(Vector3::new(3, 3 + 9, 0)).unwrap();
    assert!(is_in);

    let is_in = c.is_point_in(Vector3::new(3, 3 + 11, 0)).unwrap();
    assert!(!is_in);
}
