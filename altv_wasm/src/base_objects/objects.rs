use std::collections::{HashSet, HashMap};
use altv_wasm_shared::{BaseObjectPtr, BaseObjectType};
use crate::__imports;

use super::kind::BaseObjectKind;

#[derive(Debug)]
pub(crate) struct ObjectData {
    ref_count: u32,
    ty: BaseObjectType,
    pub(crate) kind: BaseObjectKind,
}

pub(crate) type AllBaseObjects = HashMap<BaseObjectPtr, ObjectData>;

macro_rules! objects {
    ( $( $class:ident )+ ) => { paste::paste! {
        #[derive(Debug, Default)]
        pub(crate) struct BaseObjectManager {
            pub(crate) all: AllBaseObjects,
        $(
            pub(crate) [<$class:snake>]: HashSet<BaseObjectPtr>,
        )+
        }

        impl BaseObjectManager {
            pub(crate) fn on_create(&mut self, ptr: BaseObjectPtr, ty: BaseObjectType, kind: BaseObjectKind) {
                logger::debug!("on_create base object ty: {ty:?}");

                self.all.insert(ptr, ObjectData { ref_count: 0, ty, kind });

                match ty { $(
                    BaseObjectType::$class => {
                        self.[<$class:snake>].insert(ptr);
                    }
                )+
                    _ => {
                        logger::error!("Create unknown base object: {ptr:?} {ty:?}");
                    }
                }
            }

            pub(crate) fn on_destroy(&mut self, ptr: BaseObjectPtr) {
                let data = self.all.remove(&ptr).unwrap();
                let ty = data.ty;

                logger::debug!("on_destroy base object ty: {ty:?}");

                match ty { $(
                    BaseObjectType::$class => {
                        self.[<$class:snake>].remove(&ptr);
                    }
                )+
                    _ => {
                        logger::error!("Create unknown base object: {ptr:?} {ty:?}");
                    }
                }
            }

            pub(crate) fn add_ref(all: &mut AllBaseObjects, ptr: BaseObjectPtr) {
                logger::debug!("add_ref {ptr:?}");

                all.entry(ptr).and_modify(|data| data.ref_count += 1);
            }

            pub(crate) fn remove_ref(&mut self, ptr: BaseObjectPtr) {
                logger::debug!("remove_ref {ptr:?}");

                let data = self.all.get(&ptr).unwrap();

                if data.ref_count == 1 {
                    self.on_destroy(ptr);
                    __imports::destroy_base_object(ptr);
                } else {
                    self.all.entry(ptr).and_modify(|data| data.ref_count -= 1);
                }
            }
        }

    $(
        pub(crate) mod [<$class:snake>] {
            use super::super::base::private::BaseObject;

            #[derive(Debug)]
            pub struct [<$class Type>];
            pub type $class = BaseObject<[<$class Type>]>;
        }
    )+
    } };
}

objects!(
    Vehicle
    LocalVehicle
);
