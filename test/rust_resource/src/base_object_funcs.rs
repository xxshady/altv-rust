use altv::{AnyBaseObject, BaseObjectType};

use crate::helpers::destroy_all_base_objects;

pub(crate) fn test_base_object_funcs() {
    destroy_all_base_objects();
    altv::log!("############# all");
    all();
    destroy_all_base_objects();
    altv::log!("############# get_by_id");
    get_by_id();
    destroy_all_base_objects();
    altv::log!("############# all_count");
    all_count();
}

fn all() {
    let orig_marker = altv::Marker::new(altv::MarkerType::Markerarrow, 0, 0);

    let marker_at = |idx: usize| {
        let objects = altv::base_object::all();
        let obj = objects.get(idx).unwrap();
        match obj {
            altv::AnyBaseObject::Marker(m) => m.clone(),
            _ => unreachable!(),
        }
    };

    let m = marker_at(0);
    dbg!(m.id().unwrap(), orig_marker.id().unwrap());
    assert_eq!(m, orig_marker);

    let orig_marker2 = altv::Marker::new(altv::MarkerType::Markerarrow, 0, 0);
    dbg!(m.id().unwrap(), orig_marker2.id().unwrap());

    assert_ne!(m, orig_marker2);
}

fn get_by_id() {
    let orig_marker = altv::Marker::new(altv::MarkerType::Markerarrow, 0, 0);
    let id = dbg!(orig_marker.id()).unwrap();
    let obj = altv::base_object::get_by_id(BaseObjectType::Marker, id);
    let marker = assert_marker(obj);
    dbg!(marker.id().unwrap(), orig_marker.id().unwrap());
    assert_eq!(marker, orig_marker);

    altv::log!("after destroy");
    orig_marker.destroy().unwrap();

    let assert_marker_is_none = || {
        let none = dbg!(altv::base_object::get_by_id(BaseObjectType::Marker, id));
        dbg!(&none);
        assert!(none.is_none());
    };

    assert_marker_is_none();

    let orig_blip = altv::Blip::new_point(0);

    let none = altv::base_object::get_by_id(BaseObjectType::Marker, orig_blip.id().unwrap());
    dbg!(&none);
    assert!(none.is_none());

    let b = dbg!(altv::base_object::get_by_id(
        BaseObjectType::Blip,
        orig_blip.id().unwrap()
    ))
    .unwrap();
    match b {
        AnyBaseObject::Blip(b) => {
            dbg!(b.id().unwrap(), orig_blip.id().unwrap());
            assert_eq!(b, orig_blip);
        }
        _ => unreachable!(),
    }
}

fn assert_marker(obj: Option<AnyBaseObject>) -> altv::MarkerContainer {
    match obj.unwrap() {
        altv::AnyBaseObject::Marker(m) => m,
        _ => unreachable!(),
    }
}

fn all_count() {
    assert_eq!(dbg!(altv::base_object::all_count()), 0);

    altv::Marker::new(altv::MarkerType::Markerarrow, 0, 0);
    altv::Marker::new(altv::MarkerType::Markerarrow, 0, 0);
    altv::Marker::new(altv::MarkerType::Markerarrow, 0, 0);

    assert_eq!(dbg!(altv::base_object::all_count()), 3);
}
