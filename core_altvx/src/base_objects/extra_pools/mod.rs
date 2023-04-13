mod entity;
pub(crate) use entity::{get_entity_by_id, EntityPool};
pub use entity::{Entity, EntityId, EntityRawPtr};

#[derive(Debug, Default)]
pub struct ExtraPool<T> {
    base_objects: T,
}

#[derive(Debug, Default)]
pub struct ExtraPools {
    pub entity: EntityPool,
}

pub mod wrappers {
    use crate::SomeResult;

    use super::{
        super::*,
        entity::{base_ptr_to_entity_raw_ptr, EntityRawPtr},
    };
    use objects::AnyBaseObject;
    use player::PlayerContainer;
    use vehicle::VehicleContainer;

    macro_rules! extra_pool_enum {
        (@internal $any_name: ident, $name: ident, $raw_ptr_type: ty: [ $( $variant: ident, $container: ty; )+ ]) => {
            #[derive(Debug)]
            pub enum $any_name { $(
                $variant($container),
            )+ }

            impl $any_name {
                pub(crate) fn raw_ptr(&self) -> SomeResult<$raw_ptr_type> {
                    match self { $(
                        $any_name::$variant(e) => base_ptr_to_entity_raw_ptr(e.base_ptr()?),
                    )+}
                }
            }

            impl TryFrom<AnyBaseObject> for AnyEntity {
                type Error = anyhow::Error;
                fn try_from(value: AnyBaseObject) -> Result<Self, Self::Error> {
                    Ok(match value {
                    $(
                        AnyBaseObject::$variant(e) => AnyEntity::$variant(e),
                    )+
                        base_object => anyhow::bail!("cannot convert: {base_object:?} to AnyEntity"),
                    })
                }
            }
        };

        ($name: ident, $raw_ptr_type: ty: [ $( $variant: ident, $container: ty; )+ ]) => {
            paste::paste! {
                extra_pool_enum!(@internal [<Any $name>], $name, $raw_ptr_type: [ $( $variant, $container; )+ ]);
            }
        };
    }

    extra_pool_enum!(Entity, EntityRawPtr: [
        Player, PlayerContainer;
        Vehicle, VehicleContainer;
    ]);

    impl From<VehicleContainer> for AnyEntity {
        fn from(value: VehicleContainer) -> Self {
            AnyEntity::Vehicle(value)
        }
    }

    impl From<PlayerContainer> for AnyEntity {
        fn from(value: PlayerContainer) -> Self {
            AnyEntity::Player(value)
        }
    }
}
