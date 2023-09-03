use std::collections::{HashSet, hash_set};

use altv_wasm_shared::{BaseObjectPtr, BaseObjectType};

macro_rules! objects {
    ( $( $ty:ident )+ ) => { paste::paste! {
        #[derive(Debug, Default)]
        pub(crate) struct BaseObjectManager {
            $( [<$ty:snake>]: HashSet<BaseObjectPtr>, )+
        }

        impl BaseObjectManager {
            $(
                pub(crate) fn [<$ty:snake _iter>](&self) -> hash_set::Iter<BaseObjectPtr> {
                    self.[<$ty:snake>].iter()
                }
            )+

            pub(crate) fn on_create(&mut self, ptr: BaseObjectPtr, ty: BaseObjectType) {
                logger::debug!("on_create base object ty: {ty:?}");

                match ty { $(
                    BaseObjectType::$ty => {
                        self.[<$ty:snake>].insert(ptr);
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
                    BaseObjectType::$ty => {
                        self.[<$ty:snake>].remove(&ptr);
                    }
                )+
                    _ => {
                        logger::error!("Create unknown base object: {ptr:?} {ty:?}");
                    }
                }
            }
        }

        $(
            macro_rules! [<__assert_ $ty:snake _is_valid>] {
                ($object:ident) => {
                    let valid = State::with_base_objects_ref(|base_objects, _| {
                        base_objects.[<$ty:snake _iter>]().any(|ptr| *ptr == $object.ptr)
                    });
                    assert!(
                        valid,
                        "{} instance is invalid (perhaps it was destroyed in another script resource?)",
                        stringify!($ty)
                    );
                };
            }
            pub(crate) use [<__assert_ $ty:snake _is_valid>] as [<assert_ $ty:snake _is_valid>];
        )+
    } };
}

objects!(
    Vehicle
    LocalVehicle
);
