mod entity;
pub(crate) use entity::{get_entity_by_id, EntityPool};
pub use entity::{Entity, EntityRawPtr, SyncId};

#[derive(Debug, Default)]
pub struct ExtraPool<T> {
    base_objects: T,
}

#[derive(Debug, Default)]
pub struct ExtraPools {
    pub entity: EntityPool,
}

pub(self) mod wrappers {
    use super::{super::*, entity::EntityRawPtr};
    use crate::{world_object::WorldObjectRawPtr, SomeResult};
    use inherit_ptrs::traits::{Entity, WorldObject};
    use objects::AnyBaseObject;

    use std::rc::Rc;

    macro_rules! extra_pool_enum {
        (@internal
            $any_name:ident,
            $name:ident,
            $raw_ptr_type:ty: [
                $(
                    $variant:ident,
                    $container:ty,
                    $wrapper:ty;
                )+
            ]
        ) => {
            paste::paste! {
                #[derive(Debug)]
                pub enum $any_name { $(
                    $variant($container),
                )+ }

                impl $any_name {
                    #[allow(dead_code)]
                    pub(crate) fn raw_ptr(&self) -> SomeResult<$raw_ptr_type> {
                        match self { $(
                            $any_name::$variant(e) => Ok(e.inherit_ptrs()?.[<$name:snake>]()),
                        )+}
                    }
                }

                impl TryFrom<AnyBaseObject> for $any_name {
                    type Error = anyhow::Error;
                    fn try_from(value: AnyBaseObject) -> Result<Self, Self::Error> {
                        Ok(match value {
                        $(
                            AnyBaseObject::$variant(e) => $any_name::$variant(e),
                        )+
                            base_object => anyhow::bail!("cannot convert: {base_object:?} to {}", stringify!($any_name)),
                        })
                    }
                }

            $(
                impl From<$container> for $any_name {
                    fn from(value: $container) -> Self {
                        $any_name::$variant(value)
                    }
                }

                // TODO: refactor this shit, this only needed for meta
                impl From<Rc<$wrapper>> for $any_name {
                    fn from(value: Rc<$wrapper>) -> Self {
                        $any_name::$variant(BaseObjectContainer(value))
                    }
                }
            )+
            }
        };

        ($name:ident, $raw_ptr_type:ty: [ $( $variant:ident, $base_object_mod:path; )+ ]) => {
            paste::paste! {
                extra_pool_enum!(@internal
                    [<Any $name>],
                    $name,
                    $raw_ptr_type: [
                        $(
                            $variant,
                            $base_object_mod::[<$variant Container>],
                            $base_object_mod::$variant;
                        )+
                    ]
                );
            }
        };
    }

    extra_pool_enum!(Entity, EntityRawPtr: [
        Player, player;
        Vehicle, vehicle;
        Ped, ped;
        NetworkObject, network_object;
    ]);

    extra_pool_enum!(WorldObject, WorldObjectRawPtr: [
        Player, player;
        Vehicle, vehicle;
        Ped, ped;
        NetworkObject, network_object;
    ]);
}

pub use wrappers::*;
