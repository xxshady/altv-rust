use crate::base_objects::extra_pools::ExtraPools;
use std::ptr::NonNull;
use std::{cell::RefMut, fmt::Debug};

use super::{BaseObjectContainer, BaseObjectManager, BaseObjectWrapper};
use crate::sdk;

macro_rules! base_objects {
    (@internal $(
        $manager_name_snake: ident
        $name_struct: ident
        $name_container: ident
        $name_ptr: ident
        $manager_name: ident: [
            $base_type: path,
            $( $extra_pool: ident )?
        ],
    )+ ) => {
        paste::paste! {
        $(
            pub mod $manager_name_snake {
                use super::*;
                pub type $name_struct = sdk::alt::[<I $manager_name>];
                #[doc = "[Methods](struct.BaseObjectWrapper.html#impl-BaseObjectWrapper<I" $manager_name ">)"]
                pub type $manager_name = BaseObjectWrapper<$name_struct>;
                pub type $name_container = BaseObjectContainer<$name_struct>;
                #[allow(dead_code)]
                pub type $name_ptr = NonNull<$name_struct>;

                impl Debug for $manager_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{} {{ ... }}", stringify!($manager_name))
                    }
                }

                #[macro_export]
                macro_rules! [<__ $manager_name_snake _remove_from_pool>] {
                    ($base_object: expr) => {
                        $crate::resource::Resource::with_base_objects_mut(|mut v, _| -> $crate::VoidResult {
                            v.[<$manager_name_snake>].remove($base_object.ptr()?)?;
                            $(
                                $crate::resource::Resource::with_extra_base_object_pools_mut(|mut v, _| -> VoidResult {
                                    use $crate::base_objects::BasePtr;
                                    v.[<$extra_pool:snake>].remove($base_object.base_ptr()?);
                                    Ok(())
                                })?;
                            )?
                            Ok(())
                        })
                    };
                }

                #[allow(unused_imports)]
                pub(crate) use [<__ $manager_name_snake _remove_from_pool>] as remove_from_pool;

                #[macro_export]
                macro_rules! [<__ $manager_name_snake _add_to_pool>] {
                    ($ptr: expr) => {
                        $crate::resource::Resource::with_base_objects_mut(|mut v, _| {
                            let base_ptr = std::ptr::NonNull::new(unsafe {
                                $crate::sdk::$manager_name_snake::to_base_object($ptr.as_ptr())
                            }).unwrap();

                            let container = Self::_new($ptr, base_ptr);
                            v.[<$manager_name_snake>].add($ptr, container.clone());

                            $(
                                $crate::resource::Resource::with_extra_base_object_pools_mut(|mut v, _| {
                                    v.[<$extra_pool:snake>].add(
                                        $crate::base_objects::extra_pools::wrappers::[<Any $extra_pool>]::$manager_name(
                                            container.clone()
                                        )
                                    );
                                });
                            )?

                            container
                        })
                    };
                }

                #[allow(unused_imports)]
                pub(crate) use [<__ $manager_name_snake _add_to_pool>] as add_to_pool;
            }
        )+

            #[derive(Debug)]
            pub enum AnyBaseObject { $(
                $manager_name($manager_name_snake::$name_container),
            )+ }

            #[derive(Default)]
            pub struct Store {
            $(
                pub(crate) $manager_name_snake: BaseObjectManager<$manager_name_snake::$name_struct>,
            )+
            }

            impl Store {
                pub fn on_create(
                    &mut self,
                    base_ptr: NonNull<sdk::alt::IBaseObject>,
                    base_object_type: altv_sdk::BaseObjectType,
                    mut extra_pools: RefMut<ExtraPools>,
                ) {
                    match base_object_type {
                    $(
                        $base_type => {
                            let ptr = NonNull::new(unsafe { sdk::base_object::[<to_ $manager_name_snake>](base_ptr.as_ptr()) }).unwrap();
                            if self.[<$manager_name_snake>].has(ptr) {
                                logger::debug!("base object: {base_object_type:?} {ptr:?} already added");
                                return;
                            }

                            let container = $manager_name_snake::$manager_name::_new(ptr, base_ptr);

                            self.[<$manager_name_snake>].add(
                                ptr,
                                container.clone(),
                            );

                        $(
                            extra_pools.[<$extra_pool:snake>].add(
                                super::extra_pools::wrappers::[<Any $extra_pool>]::$manager_name(container)
                            );
                        )?
                        }
                    )+
                        _ => {
                            logger::error!("unknown base object type: {base_object_type:?}")
                        }
                    }
                }

                pub fn on_remove(
                    &mut self,
                    base_ptr: NonNull<sdk::alt::IBaseObject>,
                    base_object_type: altv_sdk::BaseObjectType,
                    mut extra_pools: RefMut<ExtraPools>,
                ) {
                    match base_object_type {
                    $(
                        $base_type => {
                            let ptr = NonNull::new(unsafe { sdk::base_object::[<to_ $manager_name_snake>](base_ptr.as_ptr()) }).unwrap();
                            // TEST unwrap
                            self.[<$manager_name_snake>].remove_externally(ptr).unwrap();
                        $(
                            extra_pools.[<$extra_pool:snake>].remove(base_ptr);
                        )?
                        }
                    )+
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
                            let ptr = NonNull::new(unsafe { sdk::base_object::[<to_ $manager_name_snake>](base_ptr.as_ptr()) }).unwrap();
                            let base_object = self.[<$manager_name_snake>].get_by_ptr(ptr);
                            if let Some(base_object) = base_object {
                                Some(AnyBaseObject::$manager_name(base_object))
                            } else {
                                logger::debug!("base object type: {base_object_type:?} is not in pool");
                                None
                            }
                        }
                    )+
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
        $manager_name: ident: [
            $base_type: path,
            $( $extra_pool: ident )?
        ],
    )+ ) => {
        paste::paste! {
            base_objects!(@internal $(
                [<$manager_name:snake>]
                [<$manager_name Struct>]
                [<$manager_name Container>]
                [<$manager_name MutPtr>]
                $manager_name: [
                    $base_type,
                    $( $extra_pool )?
                ],
            )+ );
        }
    };
}

base_objects!(
    ColShape: [
        altv_sdk::BaseObjectType::Colshape,
    ],
    Vehicle: [
        altv_sdk::BaseObjectType::Vehicle,
        Entity
    ],
    Player: [
        altv_sdk::BaseObjectType::Player,
        Entity
    ],
    VirtualEntity: [
        altv_sdk::BaseObjectType::VirtualEntity,
    ],
    VirtualEntityGroup: [
        altv_sdk::BaseObjectType::VirtualEntityGroup,
    ],
    Blip: [
        altv_sdk::BaseObjectType::Blip,
    ],
    VoiceChannel: [
        altv_sdk::BaseObjectType::VoiceChannel,
    ],
    Marker: [
        altv_sdk::BaseObjectType::Marker,
    ],
);

impl Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "base objects Store")
    }
}

#[derive(Debug, Default)]
pub struct PendingDestroyOrCreation;
