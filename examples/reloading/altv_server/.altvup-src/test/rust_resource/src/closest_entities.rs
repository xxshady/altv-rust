use altv::enumflags2::BitFlag;

use crate::helpers::destroy_all_base_objects;

pub const CLOSEST_ENTITIES_TEST_TIMER: u64 = 1000;

pub fn test_closest_entities() {
  destroy_all_base_objects();

  altv::Object::new("prop_bench_04", 5, 0).unwrap();
  altv::Vehicle::new("sultan", 0, 0).unwrap();
  altv::Ped::new("player_zero", 3, 0).unwrap();

  altv::set_timeout(
    || {
      let types = altv::BaseObjectFilter::all();
      dbg!(types);

      let objects = altv::get_closest_entities(0, 100, 0, 10, types, Default::default());
      dbg!(&objects);
      assert_eq!(objects.len(), 3);

      let objects =
        altv::get_closest_entities(0, 100, 0, 10, types, altv::ClosestEntitiesOrder::Asc);
      dbg!(&objects);

      let objects =
        altv::get_closest_entities(0, 100, 0, 10, types, altv::ClosestEntitiesOrder::Desc);
      dbg!(&objects);

      let objects = altv::get_closest_entities(
        0,
        100,
        0,
        10,
        altv::BaseObjectFilter::Vehicle,
        Default::default(),
      );
      dbg!(&objects);

      let objects = altv::get_closest_entities(
        0,
        100,
        0,
        10,
        altv::BaseObjectFilter::Vehicle | altv::BaseObjectFilter::Ped,
        Default::default(),
      );
      dbg!(&objects);
    },
    CLOSEST_ENTITIES_TEST_TIMER,
  );
}
