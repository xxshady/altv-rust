#[allow(clippy::module_inception)]
mod base_object;
pub(crate) mod extra_pools;
mod manager;
mod objects;

pub(crate) use base_object::BaseObject;
pub(crate) use base_object::BaseObjectContainer;
pub(crate) use base_object::BasePtr;
pub use base_object::ValidBaseObject;
pub(crate) use manager::BaseObjectManager;
pub(crate) use objects::Store;

pub(crate) use objects::AnyBaseObject;
pub(crate) use objects::PendingDestroyOrCreation;

pub(crate) use objects::col_shape;
pub(crate) use objects::player;
pub(crate) use objects::vehicle;
