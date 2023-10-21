use std::rc::Rc;

use super::{
    base_impl::inherit_ptrs::BaseObjectInheritPtrs, object, objects::AnyBaseObject, ped, player,
    col_shape_circle, vehicle, BaseObjectContainer, checkpoint,
};
use crate::SomeResult;

mod entity;
pub use entity::{Entity, EntityRawPtr, SyncId};

mod world_object;
pub use world_object::{WorldObject, WorldObjectRawPtr};

mod col_shape;
pub use col_shape::{ColShape, ColShapeRawPtr, ColShapeNonNull};

macro_rules! extra_pool_enum {
    (@internal
        $any_name:ident,
        $name:ident,
        $raw_ptr_type:ty: [ $(
            $variant:ident,
            $container:ty,
            $wrapper:ty;
        )+ ]
    ) => {
        paste::paste! {
            #[derive(Debug)]
            pub enum $any_name { $(
                $variant($container),
            )+ }

            impl $any_name {
                #[allow(dead_code)]
                pub(crate) fn raw_ptr(&self) -> SomeResult<$raw_ptr_type> {
                    use super::inherit_ptrs::traits::*;
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
    Object, object;
]);

extra_pool_enum!(WorldObject, WorldObjectRawPtr: [
    Player, player;
    Vehicle, vehicle;
    Ped, ped;
    Object, object;
]);

extra_pool_enum!(ColShape, ColShapeRawPtr: [
    ColShapeCircle, col_shape_circle;
    Checkpoint, checkpoint;
]);
