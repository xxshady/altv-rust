use std::{fmt::Debug, ptr::NonNull, rc::Rc};

use super::{
    base_impl::mvalue::impl_deserialize_for,
    extra_pools::{Entity, WorldObject},
    pool_funcs::BaseObjectPoolFuncs,
    BaseObjectContainer, BaseObjectId, BaseObjectManager, BaseObjectWrapper,
};
use crate::{col_shape::ColShapy, sdk};

macro_rules! base_objects {
    (@internal $(
        $manager_name_snake:ident
        $name_struct:ident
        $name_container:ident
        $name_ptr:ident
        $manager_name_rc:ident
        $manager_name:ident: [
            $base_type:path,
            $(
                @inherit_classes: $inherit_ptrs_struct:ty, [ $(
                    $impl_trait:ty,
                )+ ],
            )?
        ],
    )+ ) => {
        paste::paste! {
        $(
            pub mod $manager_name_snake {
                use super::*;

                pub type $name_struct = sdk::alt::[<I $manager_name>];

                #[doc = "[Implementation](struct.BaseObjectWrapper.html#" [<$manager_name:lower>] "-implementation)"]
                pub type $manager_name = BaseObjectWrapper<
                    $name_struct
                    $(, $crate::base_objects::inherit_ptrs::$inherit_ptrs_struct )?
                >;

                // TODO: refactor this shit, this only needed for meta
                #[allow(dead_code)]
                pub(crate) type $manager_name_rc = Rc<$manager_name>;

                pub type $name_container = BaseObjectContainer<
                    $name_struct
                    $(, $crate::base_objects::inherit_ptrs::$inherit_ptrs_struct )?
                >;

                #[allow(dead_code)]
                pub type $name_ptr = NonNull<$name_struct>;

                impl Debug for $name_container {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{} {{ ... }}", stringify!($manager_name))
                    }
                }

                impl BaseObjectPoolFuncs<$name_container> for $manager_name {
                    fn all() -> Vec<$name_container> {
                        $crate::resource::Resource::with_base_objects_ref(|v, _| v.[<$manager_name_snake>].all())
                    }

                    fn all_count() -> usize {
                        $crate::resource::Resource::with_base_objects_ref(|v, _| v.[<$manager_name_snake>].all_count())
                    }

                    fn get_by_id(id: BaseObjectId) -> Option<$name_container> {
                        $crate::resource::Resource::with_base_objects_ref(|v, _| {
                            v.[<$manager_name_snake>].get_by_id(id)
                        })
                    }
                }
            $( $(
                impl $impl_trait <$crate::base_objects::inherit_ptrs::$inherit_ptrs_struct> for $manager_name {}
            )+ )?

                impl_deserialize_for!($name_container, $manager_name, [<$manager_name_snake _mvalue_deserialize_impl>]);

                #[macro_export]
                macro_rules! [<__ $manager_name_snake _remove_from_pool>] {
                    ($base_object:expr) => {
                        $crate::resource::Resource::with_base_objects_mut(|mut v, _| -> $crate::VoidResult {
                            use $crate::base_objects::BasePtr;
                            let base_ptr = $base_object.base_ptr()?;
                            v.[<$manager_name_snake>].remove(base_ptr, $base_object.ptr()?)?;
                            Ok(())
                        })
                    };
                }

                #[allow(unused_imports)]
                pub(crate) use [<__ $manager_name_snake _remove_from_pool>] as remove_from_pool;

                #[macro_export]
                macro_rules! [<__ $manager_name_snake _add_to_pool>] {
                    ($ptr:expr) => {
                        $crate::resource::Resource::with_base_objects_mut(|mut v, _| {
                            let base_ptr = std::ptr::NonNull::new(unsafe {
                                $crate::sdk::$manager_name_snake::to_base_object($ptr.as_ptr())
                            }).unwrap();

                            let inherit_ptrs = $crate::helpers::if_not!(
                                ($(
                                    unsafe { $crate::base_objects::inherit_ptrs::$inherit_ptrs_struct::new(base_ptr.as_ptr()) }
                                )?) {}
                            );

                            let container = Self::_new($ptr, base_ptr, inherit_ptrs);

                            v.[<$manager_name_snake>].add(base_ptr, $ptr, container.clone());

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
                pub(crate) $manager_name_snake: BaseObjectManager<
                    $manager_name_snake::$name_struct
                    $(, $crate::base_objects::inherit_ptrs::$inherit_ptrs_struct )?
                >,
            )+
            }

            impl Store {
                pub fn on_create(
                    &mut self,
                    base_ptr: NonNull<sdk::alt::IBaseObject>,
                    base_object_type: altv_sdk::BaseObjectType,
                ) {
                    match base_object_type {
                    $(
                        $base_type => {
                            let ptr = $crate::helpers::base_ptr_to!(base_ptr.as_ptr(), $manager_name_snake);
                            if self.[<$manager_name_snake>].has(ptr) {
                                logger::debug!("base object: {base_object_type:?} {ptr:?} already added");
                                return;
                            }

                            let inherit_ptrs = $crate::helpers::if_not!(
                                ($(unsafe {
                                    $crate::base_objects::inherit_ptrs::$inherit_ptrs_struct::new(base_ptr.as_ptr())
                                })?) {}
                            );

                            let container = $manager_name_snake::$manager_name::_new(
                                ptr,
                                base_ptr,
                                inherit_ptrs
                            );

                            self.[<$manager_name_snake>].add(
                                base_ptr,
                                ptr,
                                container.clone(),
                            );
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
                ) {
                    match base_object_type {
                    $(
                        $base_type => {
                            let ptr = $crate::helpers::base_ptr_to!(base_ptr.as_ptr(), $manager_name_snake);
                            self.[<$manager_name_snake>].remove_externally(base_ptr, ptr).unwrap();
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
                            let ptr = $crate::helpers::base_ptr_to!(base_ptr.as_ptr(), $manager_name_snake);
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

                pub fn get_by_id(
                    &self,
                    base_object_type: altv_sdk::BaseObjectType,
                    id: BaseObjectId,
                ) -> Option<AnyBaseObject> {
                    match base_object_type {
                    $(
                        $base_type => {
                            let base_object = self.[<$manager_name_snake>].get_by_id(id);
                            if let Some(base_object) = base_object {
                                Some(AnyBaseObject::$manager_name(base_object))
                            } else {
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

                pub fn all(&self) -> Vec<AnyBaseObject> {
                    let capacity = self.all_count();
                    let mut objects = Vec::with_capacity(capacity);
                $(
                    let objects_of_type = self.[<$manager_name_snake>].all();
                    objects.extend(&mut objects_of_type.into_iter().map(AnyBaseObject::$manager_name));
                )+
                    objects
                }

                pub fn all_count(&self) -> usize {
                    let mut count = 0;
                $(
                    count += self.[<$manager_name_snake>].all_count();
                )+
                    count
                }
            }
        }
    };

    ( $(
        $manager_name:ident: [
            $base_type:path,
            $(
                @inherit_classes: $inherit_ptrs_struct:ty, [ $(
                    $impl_trait:ty,
                )+ ],
            )?
        ],
    )+ ) => {
        paste::paste! {
            base_objects!(@internal $(
                [<$manager_name:snake>]
                [<$manager_name Struct>]
                [<$manager_name Container>]
                [<$manager_name MutPtr>]
                [<$manager_name Rc>]
                $manager_name: [
                    $base_type,
                    $( @inherit_classes: $inherit_ptrs_struct, [ $(
                        $impl_trait,
                    )+ ], )?
                ],
            )+ );
        }
    };
}

base_objects!(
    ColShape: [
        altv_sdk::BaseObjectType::Colshape,
        @inherit_classes: WorldColShape, [
            WorldObject,
            ColShapy,
        ],
    ],
    Vehicle: [
        altv_sdk::BaseObjectType::Vehicle,
        @inherit_classes: WorldEntity, [
            WorldObject,
            Entity,
        ],
    ],
    Player: [
        altv_sdk::BaseObjectType::Player,
        @inherit_classes: WorldEntity, [
            WorldObject,
            Entity,
        ],
    ],
    Ped: [
        altv_sdk::BaseObjectType::Ped,
        @inherit_classes: WorldEntity, [
            WorldObject,
            Entity,
        ],
    ],
    NetworkObject: [
        altv_sdk::BaseObjectType::NetworkObject,
        @inherit_classes: WorldEntity, [
            WorldObject,
            Entity,
        ],
    ],
    VirtualEntity: [
        altv_sdk::BaseObjectType::VirtualEntity,
        @inherit_classes: WorldObject, [
            WorldObject,
        ],
    ],
    VirtualEntityGroup: [
        altv_sdk::BaseObjectType::VirtualEntityGroup,
    ],
    Blip: [
        altv_sdk::BaseObjectType::Blip,
        @inherit_classes: WorldObject, [
            WorldObject,
        ],
    ],
    VoiceChannel: [
        altv_sdk::BaseObjectType::VoiceChannel,
    ],
    Marker: [
        altv_sdk::BaseObjectType::Marker,
        @inherit_classes: WorldObject, [
            WorldObject,
        ],
    ],
    Checkpoint: [
        altv_sdk::BaseObjectType::Checkpoint,
        @inherit_classes: WorldColShape, [
            WorldObject,
            ColShapy,
        ],
    ],
    ConnectionInfo: [
        altv_sdk::BaseObjectType::ConnectionInfo,
    ],
);

impl Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "base objects Store")
    }
}

#[derive(Debug, Default)]
pub struct PendingDestroyOrCreation;
