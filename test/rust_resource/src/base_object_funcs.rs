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
    let orig_blip = altv::Blip::new_global_point(0).unwrap();

    let blip_at = |idx: usize| {
        let objects = altv::base_object::all();
        let obj = objects.get(idx).unwrap();
        match obj {
            altv::AnyBaseObject::Blip(m) => m.clone(),
            _ => unreachable!(),
        }
    };

    let b = blip_at(0);
    dbg!(b.id().unwrap(), orig_blip.id().unwrap());
    assert_eq!(b, orig_blip);

    let orig_blip2 = altv::Blip::new_global_point(0).unwrap();
    dbg!(b.id().unwrap(), orig_blip2.id().unwrap());

    assert_ne!(b, orig_blip2);
}

fn get_by_id() {
    let orig_blip = altv::Blip::new_global_point(0).unwrap();
    let id = dbg!(orig_blip.id()).unwrap();
    let obj = altv::base_object::get_by_id(BaseObjectType::Blip, id);
    let blip = assert_blip(obj);
    dbg!(blip.id().unwrap(), orig_blip.id().unwrap());
    assert_eq!(blip, orig_blip);

    altv::log!("after destroy");
    orig_blip.destroy().unwrap();

    let assert_blip_is_none = || {
        let none = dbg!(altv::base_object::get_by_id(BaseObjectType::Blip, id));
        dbg!(&none);
        assert!(none.is_none());
    };

    assert_blip_is_none();

    let orig_blip2 = altv::Blip::new_global_point(0).unwrap();

    let none = altv::base_object::get_by_id(BaseObjectType::Marker, orig_blip2.id().unwrap());
    dbg!(&none);
    assert!(none.is_none());

    let b = dbg!(altv::base_object::get_by_id(
        BaseObjectType::Blip,
        orig_blip2.id().unwrap()
    ))
    .unwrap();
    match b {
        AnyBaseObject::Blip(b) => {
            dbg!(b.id().unwrap(), orig_blip2.id().unwrap());
            assert_eq!(b, orig_blip2);
        }
        _ => unreachable!(),
    }
}

fn assert_blip(obj: Option<AnyBaseObject>) -> altv::BlipContainer {
    match obj.unwrap() {
        altv::AnyBaseObject::Blip(m) => m,
        _ => unreachable!(),
    }
}

fn all_count() {
    assert_eq!(dbg!(altv::base_object::all_count()), 0);

    altv::Blip::new_global_point(0).unwrap();
    altv::Blip::new_global_point(0).unwrap();
    altv::Blip::new_global_point(0).unwrap();

    assert_eq!(dbg!(altv::base_object::all_count()), 3);
}
