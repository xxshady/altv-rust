use std::ptr::NonNull;

use crate::sdk;

use super::BaseObject;
use super::BaseObjectContainer;
use super::BaseObjectManager;

macro_rules! base_objects {
    (@internal $(
        $name_snake: ident
        $name_struct: ident
        $name_container: ident
        $name: ident: [
            $base_type: path,
            $new_func: path
        ],
    ),+ ) => {
        paste::paste! {
            $(
                pub type $name = BaseObject<$name_struct>;
                pub type $name_struct = sdk::alt::[<I $name>];
                pub type $name_container = BaseObjectContainer<$name_struct>;
            ),+

            #[derive(Debug)]
            pub enum AnyBaseObject { $(
                $name($name_container),
            )+ }

            #[derive(Default)]
            pub struct Store {
            $(
                $name_snake: BaseObjectManager<$name_struct>,
            ),+
            }

            impl Store {
            $(
                pub fn [<create_ $name_snake>](
                    &mut self,
                    ptr: NonNull<$name_struct>,
                    base_object: $name_container,
                ) {
                    self.[<$name_snake>].add(ptr, base_object);
                }

                pub fn [<remove_ $name_snake>](
                    &mut self,
                    ptr: NonNull<$name_struct>,
                ) {
                    self.[<$name_snake>].remove(ptr).unwrap();
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
                            let ptr = NonNull::new(unsafe { sdk::base_object::[<to_ $name_snake>](base_ptr.as_ptr()) }).unwrap();

                            if self.[<$name_snake>].has(ptr) {
                                logger::debug!("base object: {base_object_type:?} {ptr:?} already added");
                                return;
                            }

                            self.[<$name_snake>].add(
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
                            let ptr = NonNull::new(unsafe { sdk::base_object::[<to_ $name_snake>](base_ptr.as_ptr()) }).unwrap();
                            // TEST unwrap
                            self.[<$name_snake>].remove_externally(ptr).unwrap();
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
                            let ptr = NonNull::new(unsafe { sdk::base_object::[<to_ $name_snake>](base_ptr.as_ptr()) }).unwrap();
                            let base_object = self.[<$name_snake>].get_by_ptr(ptr);
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

    ( $(
        $name: ident: [
            $base_type: path,
            $new_func: path
        ],
    ),+ ) => {
        paste::paste! {
            base_objects!(@internal $(
                [<$name:snake>]
                [<$name Struct>]
                [<$name Container>]
                $name: [
                    $base_type,
                    $new_func
                ],
            ),+ );
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
