use crate::{
    base_object::{self, BaseObject},
    player, vehicle,
};
use std::{collections::HashMap, rc::Rc};

macro_rules! base_object_map {
    ($name: ident, $container: ty) => {
        #[derive(Debug)]
        pub struct $name {
            base_objects: HashMap<base_object::RawBaseObjectPointer, $container>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    base_objects: HashMap::new(),
                }
            }

            pub fn add_base_object(&mut self, base_object: $container) {
                self.base_objects.insert(
                    base_object.borrow().ptr().get().unwrap(),
                    Rc::clone(&base_object),
                );
            }

            pub fn remove_base_object(&mut self, raw_ptr: base_object::RawBaseObjectPointer) {
                self.base_objects.remove(&raw_ptr);
            }

            pub fn get_by_base_object_ptr(
                &self,
                raw_ptr: base_object::RawBaseObjectPointer,
            ) -> Option<$container> {
                self.base_objects.get(&raw_ptr).cloned()
            }
        }
    };
}

base_object_map!(PlayerBaseObjectMap, player::PlayerContainer);
base_object_map!(VehicleBaseObjectMap, vehicle::VehicleContainer);
