mod base_object;
pub mod data;
pub(crate) mod extra_pools;
mod manager;
pub(crate) mod meta;
mod objects;

pub(crate) use base_object::BaseObjectContainer;
pub use base_object::BaseObjectWrapper;
pub(crate) use base_object::BasePtr;
pub use base_object::ValidBaseObject;
pub(crate) use manager::BaseObjectManager;
pub(crate) use objects::Store;

pub(crate) use objects::AnyBaseObject;
pub(crate) use objects::PendingDestroyOrCreation;

pub(crate) use objects::blip;
pub(crate) use objects::col_shape;
pub(crate) use objects::marker;
pub(crate) use objects::player;
pub(crate) use objects::vehicle;
pub(crate) use objects::virtual_entity;
pub(crate) use objects::virtual_entity_group;
pub(crate) use objects::voice_channel;
