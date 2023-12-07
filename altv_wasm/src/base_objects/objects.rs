use std::collections::{HashSet, hash_set};

use altv_wasm_shared::{BaseObjectPtr, BaseObjectType};

macro_rules! objects {
    ( $( $class:ident )+ ) => { paste::paste! {
        #[derive(Debug, Default)]
        pub(crate) struct BaseObjectManager {
            pub(crate) all: HashSet<BaseObjectPtr>,
        $(
            [<$class:snake>]: HashSet<BaseObjectPtr>,
        )+
        }

        impl BaseObjectManager {
        $(
            pub(crate) fn [<$class:snake _iter>](&self) -> hash_set::Iter<BaseObjectPtr> {
                self.[<$class:snake>].iter()
            }
        )+

            pub(crate) fn on_create(&mut self, ptr: BaseObjectPtr, ty: BaseObjectType) {
                logger::debug!("on_create base object ty: {ty:?}");

                match ty { $(
                    BaseObjectType::$class => {
                        self.all.insert(ptr);
                        self.[<$class:snake>].insert(ptr);
                    }
                )+
                    _ => {
                        logger::error!("Create unknown base object: {ptr:?} {ty:?}");
                    }
                }
            }

            pub(crate) fn on_destroy(&mut self, ptr: BaseObjectPtr, ty: BaseObjectType) {
                logger::debug!("on_destroy base object ty: {ty:?}");

                match ty { $(
                    BaseObjectType::$class => {
                        self.all.remove(&ptr);
                        self.[<$class:snake>].remove(&ptr);
                    }
                )+
                    _ => {
                        logger::error!("Create unknown base object: {ptr:?} {ty:?}");
                    }
                }
            }
        }

    $(
        pub(crate) mod [<$class:snake>] {
            use super::super::base::private::{BaseObject, BaseObjectLocked};

            #[derive(Debug)]
            pub struct [<$class Type>];
            pub type $class = BaseObject<[<$class Type>]>;
            pub type [<$class Locked>]<'m, M> = BaseObjectLocked<'m, [<$class Type>], M>;
        }
    )+
    } };
}

objects!(
    Vehicle
    LocalVehicle
);
