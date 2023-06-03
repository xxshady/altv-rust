use altv::BaseObjectPoolFuncs;

use crate::helpers::destroy_all_base_objects;

pub(crate) fn test_base_object_pool_funcs() {
    altv::log!("############# get_by_id");
    get_by_id();
    altv::log!("############# all");
    all();
    altv::log!("############# all_count");
    all_count();
}

fn get_by_id() {
    let marker = altv::Marker::new(altv::MarkerType::Markerlap, 0, 0);
    let id = dbg!(marker.id()).unwrap();

    dbg!(altv::Marker::get_by_id(id).unwrap());

    let none = altv::Marker::get_by_id(id + 1);
    dbg!(&none);
    assert_eq!(none, None);

    {
        let blip = altv::Blip::new_point(0);
        let id = dbg!(blip.id()).unwrap();
        dbg!(altv::Blip::get_by_id(id));
    }

    altv::log!("get_by_id after destroy");

    marker.destroy().unwrap();

    let none = altv::Marker::get_by_id(id);
    dbg!(&none);
    assert_eq!(none, None);
}

fn all() {
    destroy_all_base_objects();
    assert_eq!(altv::Marker::all().len(), 0);
    assert_eq!(altv::Blip::all().len(), 0);

    altv::Blip::new_point(0);
    altv::Blip::new_point(0);
    altv::Blip::new_point(0);

    assert_eq!(dbg!(altv::Blip::all().len()), 3);

    altv::Blip::all().iter().for_each(|v| v.destroy().unwrap());

    assert_eq!(dbg!(altv::Blip::all().len()), 0);
}

fn all_count() {
    destroy_all_base_objects();
    assert_eq!(altv::Marker::all_count(), 0);
    assert_eq!(altv::Blip::all_count(), 0);

    altv::Blip::new_point(0);
    altv::Blip::new_point(0);
    altv::Blip::new_point(0);

    assert_eq!(dbg!(altv::Blip::all_count()), 3);

    altv::Blip::all().iter().for_each(|v| v.destroy().unwrap());

    assert_eq!(dbg!(altv::Blip::all_count()), 0);
}
