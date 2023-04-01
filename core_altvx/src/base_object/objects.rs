use std::ptr::NonNull;

use crate::sdk;

use super::BaseObject;
use super::BaseObjectContainer;
use super::BaseObjectManager;

macro_rules! base_objects {
    ( $( $name: ident: [$base_type: path, $new_func: path], ),+ ) => {
        paste::paste! {
            $(
                pub type [<$name Struct>] = sdk::alt::[<I $name>];
                pub type $name = BaseObject<[<$name Struct>]>;
                pub type [<$name Container>] = BaseObjectContainer<[<$name Struct>]>;
            ),+

            #[derive(Debug)]
            pub enum AnyBaseObject { $(
                $name([<$name Container>]),
            )+ }

            #[derive(Default)]
            pub struct Store {
            $(
                $name: BaseObjectManager<[<$name Struct>]>,
            ),+
            }

            impl Store {
            $(
                pub fn [<create_ $name:snake>](
                    &mut self,
                    ptr: NonNull<[<$name Struct>]>,
                    base_object: [<$name Container>],
                ) {
                    self.[<$name>].add(ptr, base_object);
                }

                pub fn [<remove_ $name:snake>](
                    &mut self,
                    ptr: NonNull<[<$name Struct>]>,
                ) {
                    self.[<$name>].remove(ptr).unwrap();
                }
            ),+

                pub fn on_create(
                    &mut self,
                    base_ptr: NonNull<sdk::alt::IBaseObject>,
                    base_object_type: altv_sdk::BaseObjectType,
                ) {
                    match base_object_type {
                    $(
                        $base_type => {
                            let ptr = NonNull::new(unsafe { sdk::base_object::[<to_ $name:snake>](base_ptr.as_ptr()) }).unwrap();

                            if self.[<$name>].has(ptr) {
                                logger::error!("base object: {base_object_type:?} {ptr:?} already added");
                                return;
                            }

                            self.[<$name>].add(
                                ptr,
                                $new_func(ptr, base_ptr)
                            )
                        }
                    ),+
                        _ => {
                            logger::error!("unknown base object type: {base_object_type:?}")
                        }
                    }
                }

                pub fn on_remove(
                    &mut self,
                    base_ptr: NonNull<sdk::alt::IBaseObject>,
                    base_object_type: altv_sdk::BaseObjectType,
                ) {
                    match base_object_type {
                    $(
                        $base_type => {
                            let ptr = NonNull::new(unsafe { sdk::base_object::[<to_ $name:snake>](base_ptr.as_ptr()) }).unwrap();
                            // TEST unwrap
                            self.[<$name>].remove_externally(ptr).unwrap();
                        }
                    ),+
                        _ => {
                            logger::error!("unknown base object type: {base_object_type:?}")
                        }
                    }
                }

                pub fn get_by_ptr(
                    &self,
                    base_ptr: NonNull<sdk::alt::IBaseObject>,
                ) -> Option<AnyBaseObject> {
                    let base_object_type = unsafe { altv_sdk::helpers::get_base_object_type(base_ptr.as_ptr()) };

                    match base_object_type {
                    $(
                        $base_type => {
                            let ptr = NonNull::new(unsafe { sdk::base_object::[<to_ $name:snake>](base_ptr.as_ptr()) }).unwrap();
                            let base_object = self.[<$name>].get_by_ptr(ptr);
                            if let Some(base_object) = base_object {
                                Some(AnyBaseObject::$name(base_object))
                            } else {
                                logger::debug!("base object type: {base_object_type:?} is not in pool");
                                None
                            }
                        }
                    ),+
                        _ => {
                            logger::error!("unknown base object type: {base_object_type:?}");
                            None
                        }
                    }
                }
            }
        }
    };
}

base_objects!(ColShape: [
    altv_sdk::BaseObjectType::Colshape,
    ColShape::_new
],);

impl std::fmt::Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "base objects Store")
    }
}

#[derive(Debug, Default)]
pub struct PendingDestroy;
