use crate::{base_object::BaseObject, col_shape, player, vehicle};
use std::{collections::HashMap, rc::Rc};

pub trait BaseObjectMap<Container> {
    fn add_base_object(&mut self, base_object: Container);
    fn remove_base_object(&mut self, raw_ptr: altv_sdk::IBaseObjectMutPtr);
    fn get_by_base_object_ptr(&self, raw_ptr: altv_sdk::IBaseObjectMutPtr) -> Option<Container>;
}

macro_rules! base_object_map {
    ($name: ident, $container: ty) => {
        #[derive(Debug, Default)]
        pub struct $name {
            base_objects: HashMap<altv_sdk::IBaseObjectMutPtr, $container>,
        }

        impl BaseObjectMap<$container> for $name {
            fn add_base_object(&mut self, base_object: $container) {
                self.base_objects.insert(
                    base_object.borrow().ptr().get().unwrap(),
                    Rc::clone(&base_object),
                );
            }

            fn remove_base_object(&mut self, raw_ptr: altv_sdk::IBaseObjectMutPtr) {
                self.base_objects.remove(&raw_ptr);
            }

            fn get_by_base_object_ptr(
                &self,
                raw_ptr: altv_sdk::IBaseObjectMutPtr,
            ) -> Option<$container> {
                self.base_objects.get(&raw_ptr).cloned()
            }
        }
    };
}

base_object_map!(PlayerBaseObjectMap, player::PlayerContainer);
base_object_map!(VehicleBaseObjectMap, vehicle::VehicleContainer);
base_object_map!(ColShapeBaseObjectMap, col_shape::ColShapeContainer);
